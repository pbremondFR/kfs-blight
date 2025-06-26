section .text
global reload_gdt

gdt_ptr:
	dw 0		; Define word: Size placeholder, will be set by reload_gdt
	dd 0x0800	; Define double word: Hardcoded GDT location imposed by subject at 0x00000800
reload_gdt:
	mov eax, [esp + 4]
	mov ecx, 8
	mul ecx
	dec eax
	; Store calculated size in the gdt pointer
	mov word [gdt_ptr], ax
	; Load GDT
	lgdt [gdt_ptr]
	; long-jump to set CS register to the 2nd GDT entry (kernel code segment)
	jmp 0x08:reset_segment_registers
reset_segment_registers:
	mov ax, 0x10	; Set data segment registers to kernel data segment descriptor (3rd entry, 0x10)
	mov ds, ax
	mov es, ax
	mov fs, ax
	mov gs, ax
	mov ax, 0x18	; Set stack segment register to kernel stack segment descriptor (4th entry, 0x18)
	mov ss, ax
	ret
