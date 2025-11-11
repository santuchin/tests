
def compress_string(raw: str) -> str:

	result = ''

	count = 0

	prev = None

	for char in raw:

		if not prev is None:

			if char == prev:
				count += 1
			
			else:
				result += char + str(count)
				count = 0
			
		
		prev = char

	return result
