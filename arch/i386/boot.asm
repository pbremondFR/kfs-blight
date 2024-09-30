STACK_SIZE equ 16384

section .text

global start
extern kmain

start:
  mov   esp, stack + STACK_SIZE ; create Stack
  push  0                       ; Reset Error Flags
  popf
  push  ebx                     ; Push Magic Multiboot Number
  push  eax                     ; Push Multiboot Structure Pointer
  call  kmain                   ; Call kmain (leaving asm for rust)

section .bss
stack:
  resb STACK_SIZE
