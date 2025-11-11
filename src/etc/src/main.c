#include <stdio.h>
#include <stdbool.h>

int main(void) {

	char* start = ((char*) &main) - 1000;

	char current;

	while (true) {

		current = *start; // until this causes segfault

		putchar(current);

		start += 1;
	}

	return 0;
}
