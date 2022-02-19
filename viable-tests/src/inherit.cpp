#include "shared.hpp"

interface Pet {
public:
	virtual const char* name() = 0;
};

interface Dog: public Pet {
public:
	virtual const char* speak() = 0;
};

class Pug: public Dog {
public:
	const char* thename;
	int theage;

	Pug(const char* iname, int age) {
		this->thename = iname;
		this->theage = age;
	}

	virtual const char* name() {
		return thename;
	}

	virtual const char* speak() {
		return "bark";
	}

	// Not from Dog nor Pet
	virtual int age() {
		return theage;
	}
};


extern "C" {
	Pug* getPug(const char* name, int age) {
		return new Pug(name, age);
	}
}