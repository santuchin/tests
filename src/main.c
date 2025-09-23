#include <stdio.h>
#include <unistd.h>

void fill_spiral(size_t length, int** matrix) {

	size_t number = 0;

	for (size_t index = 0; index < length; index += 1) {
		matrix[index][0] = number;
		number += 1;
	}

}

int main(void) {

	for (size_t index = 0; index < 10; index++)
		printf("%zu\n", index);

	return 0;
}
