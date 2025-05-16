STACK_SIZE equ 16384

section .text

global start
extern kmain

start:
	mov esp, stack + STACK_SIZE
	mov ebp, esp
	call kmain

section .bss
stack:
	resb STACK_SIZE
