#include <iostream>

struct twci { 
	static void hello() {
		std::cout << "hello" << std::endl;
	};
};

#define MACROONE \
	static void hello(){twci::hello();}; \
	static twci me; \


struct UnkownSpace {
	MACROONE
};

struct heeelo {
	static void heelo() {
		UnkownSpace::hello();
	}
};
