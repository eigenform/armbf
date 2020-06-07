/*
 * Bunch of nonsense; just to have some example code to work with when
 * we start actually decoding some instructions.
 */

#include <stdint.h>
typedef uint8_t u8;
typedef uint16_t u16;
typedef uint32_t u32;

extern void main();
extern void panic();

void _start() { main(); panic(); }
void panic() { while (1) {} }

int write_something(u32 x) {
	*(u32*)0xdeadbeef = x;
	if (*(u32*)0xdeadbeef != x)
		return -1;
	return 0;
}



void write_string(char *str, int len) {
	for (int i = 0; i < len; i++) {
		*(u8*)0xdeadcafe = str[i];
	}
}
void blahg() {
	char *my_string = "This is a string!";
	write_string(my_string, 32);
}

void __invalidate_icache() { 
	__builtin_arm_mcr(15, 0, 0x0, 7, 5, 0); 
}
void __update_tlb_base(u32 x) { 
	__builtin_arm_mcr(15, 0, x, 2, 0, 0);
}
u32 __read_dfault() { 
	return __builtin_arm_mrc(15, 0, 5, 0, 0);
}

u32 dp_test(u32 x, u32 y) {
	u32 foo = x * 0x5a5a5a5a;
	u32 bar = (y * y) * 3;
	u8 a, b, c;
	foo = foo - (y ^ 2);

	foo = foo - ( (x / 22) | 0x50005000);
	a = foo & 0x000000ff;


	while (1) {
		b = (bar & 0x00ff0000) >> 16;
		c = c + (y & 0xff) ^ (b | a + 0x80);
		bar = bar | (a << 24) | (b << 16 ^ a) | c;
		if (b >= 0x80) continue;
		else if (b < 0x20) {
			b = a + (x & 0xff);
			c = b + b * a;
		}
		else if (b == 0x11) {
			break;
		}
	}

	return foo;
}

void main() {
	u8 buf[0x10];
	u32 x = 0xffffffff;
	u32 a, b, c;
	u16 d = 0xdead;
	u16 e = 0xcafe;

	for (int i = 500; i > 0; i--) {
		write_something(x);
		dp_test(x, (a & b | c));
	}

	panic();
}

