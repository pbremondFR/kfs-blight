global reload_gdt

gdt_ptr:
	; TODO: Make the table size dependant on a function parameter
	dw 7 * 8 - 1
	dd 0x0800
reload_gdt:
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
