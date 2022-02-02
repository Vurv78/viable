use proc_macro::TokenStream;

use quote::{quote, format_ident, ToTokens};
use syn::{parse_macro_input, parse_quote, ItemStruct, ItemImpl, ImplItem, ImplItemMethod, Fields};

#[proc_macro_attribute]
/// Defines a struct that will act as a VTable to a C++ class.
/// It can also take data as to make sure the class functions as expected.
/// # Example
/// ```cpp
/// #define interface __declspec(novtable) class
/// interface MathEngine {
/// public:
///         virtual int add(int x, int y) = 0;
///         virtual int add2(int x, int y) = 0;
/// };
///
/// class MyEngine: public MathEngine {
/// public:
///     int bruh;
///
///     MyEngine(int b) {
///         bruh = b;
///     }
///
///     virtual int add(int x, int y) {
///         return x + y;
///     }
///
///     virtual int add2(int x, int y) {
///         return bruh + x + y;
///     }
/// };
///
/// extern "C" {
///     MyEngine* getMath(int b) {
///     return new MyEngine(b);
/// }
/// };
/// ```
/// ```rust, no_run
/// use std::os::raw::c_int;
/// use viable_impl::*;
/// extern "C" {
///     fn getMath(b: c_int) -> *mut MathIFace;
/// }
///
/// use viable_impl::vtable;
/// #[vtable]
/// struct MathIFace {
///     internal: i32,
///
///     #[offset(0)]
///     add: fn(a: c_int, b: c_int) -> c_int,
///     add2: fn(a: c_int, b: c_int) -> c_int,
/// }
///
/// pub fn main() {
///     let iface = unsafe { getMath(10) };
///     let iface = unsafe { iface.as_mut().unwrap() };
///
///     let value = iface.add2(5, 5);
///
///    assert_eq!(value, 10 + 5 + 5);
/// }
/// ```
pub fn vtable(_attr: TokenStream, item: TokenStream) -> TokenStream {
	let ast = parse_macro_input!(item as ItemStruct);

	let ident = &ast.ident;

	let mut interface: ItemImpl = parse_quote! {
		impl #ident {}
	};

	let mut fields = vec![];
	let mut count = 0usize;
	for f in &ast.fields {
		if let Some(id) = &f.ident {
			match f.ty {
				syn::Type::BareFn(ref x) => {
					// Look for #[offset(n)] attribute
					for attr in &f.attrs {
						let s = attr.path.to_token_stream();
						if s.to_string() == "offset" {
							let offset: syn::LitInt = attr.parse_args().unwrap();
							count = offset.base10_parse::<usize>().expect("Offset must be usize");
							break;
						}
					}

					let (name, ty) = (id.to_string(), x.clone());
					let ret = &ty.output;

					// Full signature with the pointer to the original class.
					let mut ty_full = ty.clone();
					ty_full.inputs.insert(0,  parse_quote! {_self: *mut Self} );

					let inputs = &ty.inputs;

					let name = format_ident!("{name}");

					let mut call: syn::ExprCall = parse_quote! {
						func(self)
					};

					for (pnum, i) in inputs.iter().enumerate() {
						let b = i.name.as_ref().map(|(i, _)| i.to_string()).unwrap_or(format!("argn{pnum}"));
						let b = format_ident!("{}", b);

						call.args.push_punct( parse_quote! { , } );
						call.args.push_value( parse_quote! { #b } );
					}

					let mut item: ImplItemMethod = parse_quote! {
						// #[offset(#count)]
						fn #name(&mut self, #inputs) #ret {
							let vtable = self.vtable as *const #ty_full;
							let func = unsafe { vtable.add(#count).read() };
						}
					};

					item.block.stmts.push( syn::Stmt::Expr( syn::Expr::Call(call) ) );
					interface.items.push( ImplItem::Method(item) );
					count += 1;
				},
				_ => {
					fields.push(f);
					//panic!("VTable fields must be bare functions!")
				},
			}
		}
	}


	let mut struc: ItemStruct = parse_quote! {
		#[repr(C)]
		struct #ident {
			vtable: *mut *mut usize,
		}
	};

	// Add data fields (non bare functions)
	for f in fields {
		if let Fields::Named(ref mut x) = struc.fields {
			x.named.push( f.to_owned() );
		}
	}

	quote! {
		#struc
		#interface
	}.into()
}

/*#[proc_macro_attribute]
pub fn offset(attr: TokenStream, item: TokenStream) -> TokenStream {
	let index = parse_macro_input!(attr as VirtualIndex).index;

	let parsed: ItemFn = syn::parse(item.clone()).unwrap();

	let foo = parsed
	.sig
	.inputs
	.iter()
	.flat_map(|arg| {
		if let FnArg::Typed(pat) = arg {
			return Some(*pat.ty.clone());
		}
		None
	})
	.collect::<Vec<Type>>();
}*/