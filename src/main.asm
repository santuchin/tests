
section .bss
	buffer resb 64
	buffer_len equ 64
	char resb 1


read equ 0
write equ 1
exit equ 60

stdin equ 0
stdout equ 1

section .text
	global _start


_start:

	mov rdx, buffer_len
	mov rsi, buffer
	mov rdi, stdin
	mov rax, read
	syscall

	mov rcx, rax

	mov rdx, 1
	mov rsi, char
	mov rdi, stdin
	mov rax, read
	syscall


	mov rax, 0
	mov rdx, buffer
	mov bl, [char]

loop:
	cmp rcx, 0
	je done
	dec rcx

	mov sil, [rdx]

	cmp bl, sil
	jne diff
	inc rax
	diff:

	inc rdx
	jmp loop

done:

	mov rdi, rax
	mov rax, exit
	syscall
