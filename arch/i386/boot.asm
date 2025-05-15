STACK_SIZE equ 16384

section .gdt_section

gdt_start:
	; NULL descriptor
	dd 0
	dd 0

	; First entry (descriptor 0x08) for code segment. Copied from http://www.brokenthorn.com/Resources/OSDev8.html for now
	dw 0xFFFF 			; limit low
	dw 0 				; base low
	db 0 				; base middle
	db 10011010b 		; access
	db 11001111b 		; granularity
	db 0 				; base high

	; Second entry (descriptor 0x10) for data segment. Copied from http://www.brokenthorn.com/Resources/OSDev8.html for now
	dw 0xFFFF 			; limit low (Same as code)
	dw 0 				; base low
	db 0 				; base middle
	db 10010010b 		; access
	db 11001111b 		; granularity
	db 0				; base high
gdt_end:
gdt_ptr:
	dw gdt_end - gdt_start - 1
	dd gdt_start

; IDT is empty for now, will be filled by kernel in Rust code
idt_ptr:
	dw 2048	; IDT size (8 bytes entries * 258 = 2048 bytes)
	dd 0	; Address 0x0. GDT is located 2048 bytes later, at 0x00000800

section .text

global start
extern kmain

start:
	cli				; Disable interrupts
	lgdt [gdt_ptr]	; Load GDT
	lidt [idt_ptr]	; Load empty IDT

	; To switch to protected mode, enable the PE bit in thr CR0 register
	mov eax, cr0
	or eax, 1
	mov cr0, eax

	; Far-jump to clear CPU pre-fetch queue & correctly set CS register
	; Jump with segment selector set to 0x8 to jump to code descriptor in GDT
	jmp 08h:protected_mode

[bits 32]
protected_mode:
	mov		ax, 0x10		; set data segments to data selector (0x10)
    mov		ds, ax
    mov		ss, ax
    mov		es, ax
    mov		fs, ax
    mov		gs, ax

	; sti				; Enable interrupts again

launch_kernel:
	mov esp, stack + STACK_SIZE
	mov ebp, esp
	call kmain

section .bss
stack:
	resb STACK_SIZE
