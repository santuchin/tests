#include <stdio.h>
#include <stdint.h>
#include <inttypes.h>

struct a {
	uint8_t a : 1;
	uint8_t b : 1;
	uint8_t c : 1;
	uint8_t d : 1;
	uint8_t e : 1;
	uint8_t f : 1;
	uint8_t g : 1;
	uint8_t h : 1;
};

struct b {
	uint8_t x;
};

int main(void) {

	printf("sizeof(a) = %zu\n", sizeof(struct a));
	printf("sizeof(b) = %zu\n", sizeof(struct b));

	struct a a;
	struct b b;

	a.a = 1;
	b.x |= 0b00000001;
	b.x &= ~0b00000000;

	printf("b.x = %" PRIu8 "\n", b.x & 0b00000001);
	
	return 0;
}
