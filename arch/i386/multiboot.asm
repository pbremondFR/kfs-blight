section .multiboot
align 8
header_start:
	dd 0xe85250d6					                                ; magic number (multiboot 2)
	dd 0							                                ; architecture 0 (protected mode i386)
	dd header_end - header_start                	                ; header length
	dd 0x100000000 - (0xe85250d6 + 0 + (header_end - header_start)) ; checksum
 align 8
 framebuffer_tag_start:
    dw  0x05                                                        ; Type: framebuffer
    dw  0x01                                                        ; Optional tag
    dd  framebuffer_tag_end - framebuffer_tag_start                 ; size
    dd  0                                                           ; Width - 0 = let the bootloader decide
    dd  0                                                           ; Height - same as above
    dd  0                                                           ; Depth  - same as above
framebuffer_tag_end:
align 8
	dw 0
	dw 0
	dd 8
header_end:
