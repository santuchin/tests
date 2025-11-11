
def log(number: int, base: int) -> int:

	count = 0

	while number < base:
		count += 1
		number /= base

	return count

def logrem(number: int, base: int) -> tuple[int, int]:

	factor = 1

	count = 0
	remainder = 0

	while not number < base:

		count += 1

		number, temp = divmod(number, base)
			
		remainder += temp * factor
		factor *= base

	
	return (count, remainder)

def log2rem(number: int) -> tuple[int, int]:

	count = 0
	remainder = 0

	while number >= base:

		number, temp = divmod(number, base)

		remainder += temp << count
		count += 1

	return (count, remainder)

base = 10

number = 112

print(number, logrem(number, base))
