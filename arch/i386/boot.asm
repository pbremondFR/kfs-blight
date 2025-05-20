STACK_SIZE equ 16384

section .text

global start
extern kmain

start:
	; Setup stack at arbitrary location in .bss
	; Make sure to be compatible with everything that can be thrown at us by aligning it
	; to 16 bytes.
	mov esp, kernel_stack + STACK_SIZE
	mov ebp, esp
	call kmain

section .bss

	; Just in case, add optional padding to align stack to 16 bytes
	align 16, db 0
kernel_stack:
	resb STACK_SIZE
