MAGIC    equ 0xE85250D6
ARCH     equ 0
LEN      equ _hdr_end - _hdr_start
CHECKSUM equ 0x100000000-(MAGIC+ARCH+LEN)

; Multiboot section

[BITS 32]
[SECTION .mbhdr]

; Those symbols are added by the linker at link-time. See linked.ld

[EXTERN _load_start]
[EXTERN _load_end]
[EXTERN _bss_end]

ALIGN 16

_hdr_start:
    ; Multiboot 1.X magic number
    dd  MAGIC

    ; Architecture
    dd  ARCH

    ; Length of the mboot header
    dd  LEN

    ; Checksum
    DD  CHECKSUM

    ; Tags

    ; Sections override
    dw  2, 0
    dd  24
    dd  _hdr_start
    dd  _load_start
    dd  _load_end
    dd  _bss_end

    ; Entry point override
    dw  3, 0
    dd  16
    dd  entry_point
    dd  0

    ; End Of Tags
    dw  0, 0
    dd  8
_hdr_end:

; Main entry point, executed directly upon loading by Grub
; For now, CPU is still in protected mode, so this is 32-bit code.

[SECTION .boot]
[GLOBAL entry_point]
[EXTERN stack]

entry_point:
    ; loads first GDT (see at the end of this file)
    mov     eax,    gdtr1
    lgdt    [eax]

    push    0x08
    push    .gdt_ready
    retf

.gdt_ready:
    mov     eax,    0x10
    mov     ds,     ax
    mov     ss,     ax
    mov     esp,    stack

    ; go to long mode
    call    setup_long_mode

    ; loads 64-bit GDT
    mov     eax,    gdtr2
    lgdt    [gdtr2]

    push    0x08
    push    .gdt2_ready
    retf

; This part runs once the CPU is in long-mode. This is 64-bit code
; This code is reponsible of setting up the environment for the kernel
; and jump to main

[BITS 64]
[EXTERN kmain]

.gdt2_ready:
    mov     eax,    0x10
    mov     ds,     ax
    mov     es,     ax
    mov     ss,     ax
    mov     rsp,    (stack + 0xFFFFFFFF80000000)

    ; Watch out when unmapping the lower zone, you will have an invalid
    ; Gdt if you're still using Gdtr2

    mov     rax,    gdtr3
    lgdt    [rax]

    ; Jump to kernel

    mov     rax, kmain
    call    rax

    cli
.hlt:
    hlt
    jmp     .hlt

; 32-bits code part that do 64 bit initialization

[BITS 32]

; Space for paging data-structures is reserved by the linker (see linker.ld)

[EXTERN pml4]
[EXTERN pdpt]
[EXTERN pagedir]

setup_long_mode:
    ; setup the paging tables
    mov     eax,    pdpt
    or      eax,    1
    mov     [pml4], eax
    mov     [pml4 + 0xFF8],     eax

    mov     eax,    pagedir
    or      eax,    1
    mov     [pdpt], eax
    mov     [pdpt + 0xFF0],     eax

    mov     dword   [pagedir],      0x000083
    mov     dword   [pagedir + 8],  0x200083
    mov     dword   [pagedir + 16], 0x400083
    mov     dword   [pagedir + 24], 0x600083

    ; load page directory in the CPU register

    mov     eax,    pml4
    mov     cr3,    eax

    ; Enable PAE

    mov     eax,    cr4
    or      eax,    1 << 5
    mov     cr4,    eax

    ; Enable Long Mode in the MSR

    mov     ecx,    0xC0000080
    rdmsr
    or      eax,    (1 << 8)
    wrmsr

    ; Enable Paging

    mov     eax,    cr0
    or      eax,    (1 << 31)
    mov     cr0,    eax

    ret

; GDT data structures

tmp_gdt:
    dq  0x0000000000000000
    dq  0x00CF9A000000FFFF
    dq  0x00CF92000000FFFF
    dq  0x0000000000000000
    dq  0x00A09A0000000000
    dq  0x00A0920000000000

gdtr1:
    dw  23
    dd  tmp_gdt

gdtr2:
    dw  23
    dd  tmp_gdt + 24
    dd  0

gdtr3:
    dw  23
    dq  tmp_gdt + 24 + 0xFFFFFFFF80000000
