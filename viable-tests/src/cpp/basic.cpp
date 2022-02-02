#define interface __declspec(novtable) class

interface MathEngine {
public:
	virtual int add(int x, int y) = 0;
	virtual int add2(int x, int y) = 0;
};

class MyEngine: public MathEngine {
public:
	int bruh;

	MyEngine(int b) {
		bruh = b;
	}

	virtual int add(int x, int y) {
		return x + y;
	}

	virtual int add2(int x, int y) {
		return bruh + x + y;
	}
};

extern "C" {
	MyEngine* getMath(int b) {
		return new MyEngine(b);
	}
};