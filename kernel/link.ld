ENTRY(entry_point)

/* base virtual address of the kernel */
VIRT_BASE = 0xFFFFFFFF80000000;

SECTIONS {
    . = 0x100000;

    /* Place multiboot header at 0x10000 as it is where Grub will be looking
     * for it.
     * Immediately followed by the boot code */
    .boot : {
        KEEP(*(.mbhdr))
        _load_start = .;
        KEEP(*(.boot))

        . = ALIGN(4096);

        /* reserve space for paging data structures */
        pml4 = .;
        . += 0x1000;
        pdpt = .;
        . += 0x1000;
        pagedir = .;
        . += 0x1000;
        . += 0x8000;

        /* stack segment for loader */
        stack = .;
    }

    /* Kernel code section is placed at his virtual address */

    . += VIRT_BASE;

    .text ALIGN(0x1000) : AT(ADDR(.text) - VIRT_BASE) {
        KEEP(*(.text))
        *(.text)
        *(.gnu.linkonce.t*)
    }

    .data ALIGN(0x1000) : AT(ADDR(.data) - VIRT_BASE) {
        KEEP(*(.data))
        *(.data)
        *(.gnu.linkonce.d*)
    }

    .rodata ALIGN(0x1000) : AT(ADDR(.rodata) - VIRT_BASE) {
        KEEP(*(.rodata))
        *(.rodata*)
        *(.gnu.linkonce.r*)
    }

    _load_end = . - VIRT_BASE;

    .bss ALIGN(0x1000) : AT(ADDR(.bss) - VIRT_BASE) {
        KEEP(*(.bss))
        KEEP(*(.bss.*))
        KEEP(*(COMMON))
        *(COMMON)
        *(.bss)
        *(.bss.*)
        *(.gnu.linkonce.b*)
    }

    _bss_end = . - VIRT_BASE;

    /DISCARD/ : {
        *(.comment)
        *(.eh_frame)
    }
}