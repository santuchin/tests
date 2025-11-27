#include <stdlib.h>

int main(void) {

	/*
	</auto/register/static/extern> type<< const><*><restrict>> <const >name
	*/

	// int const*restrict const name;

	/*
	
	never use array syntax in functions (void sample(int array[])), instead use pointer syntax (void sample(int* array)), since array syntax parameters in functions are degraded to just pointers.

	never write undefined parameter functions (void sample()), instead use void parameter functions (void sample(void))

	never declare more than one variables at the same time	

	*/

	int*restrict number;

	*number = 0;


	return 0;
}
