#include "shared.hpp"

interface MathEngine {
public:
	virtual int add(int x, int y) = 0;
	virtual int add2(int x, int y) = 0;
};

class MyEngine: public MathEngine {
public:
	int mynum;

	MyEngine(int b) {
		mynum = b;
	}

	int add(int x, int y) {
		return x + y;
	}

	int add2(int x, int y) {
		return mynum + x + y;
	}
};

extern "C" {
	MyEngine* getMath(int b) {
		return new MyEngine(b);
	}
};