
section data
	hello: "hello, world"

macro write 1
macro stdout 1
macro exit 60

section code

global _start

_start:

	copy %0, write
	copy %1, stdout
	copy %2, hello
	copy %3, ?hello
	syscall

	copy %0, exit
	copy %1, 0
	syscall
