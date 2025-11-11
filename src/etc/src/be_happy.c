
void be_happy(void) {
	volatile char* null = 0;
	*null = 0;
}

#include <stdio.h>

int main(void) {


	printf("hello world\n");
	fflush(stdout);

	be_happy();

	printf("wtf bro\n");
	fflush(stdout);
	
	return 0;
}
