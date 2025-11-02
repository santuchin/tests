#include <stdio.h>

void hello(char const* message) {
	printf("%s", message);
}

static struct {
	void (*print)(char const*);
} fs = {
	hello,
};


int main(void) {

	fs.print("hello, world\n");

	return 0;
}