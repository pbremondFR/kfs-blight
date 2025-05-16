STACK_SIZE equ 16384

section .text

global start
extern kmain

start:
	mov esp, blight_stack + STACK_SIZE
	mov ebp, esp
	call kmain

section .bss
global blight_stack
blight_stack:
	resb STACK_SIZE
