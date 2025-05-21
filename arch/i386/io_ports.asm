section .text

global inb
global inw
global inl
global outb
global outw
global outl

inb:
	mov dx, [esp+4]
	in al, dx
	ret

inw:
	mov dx, [esp+4]
	in ax, dx
	ret

inl:
	mov dx, [esp+4]
	in eax, dx
	ret

outb:
	mov dx, [esp+8]
	mov al, [esp+4]
	out dx, al
	ret

outw:
	mov dx, [esp+8]
	mov ax, [esp+4]
	out dx, ax
	ret

outl:
	mov dx, [esp+8]
	mov eax, [esp+4]
	out dx, eax
	ret
