export HOST := x86_64-elf
export ARCH := $(shell if echo $(HOST) | grep -Eq 'i[[:digit:]]86-'; then \
	echo i386 ; \
else \
	echo $(HOST) | grep -Eo '^[[:alnum:]_]*' ; \
fi)
export HOSTARCH := $(ARCH)

export NASM := nasm
export CC := $(HOST)-gcc
export XARGO := xargo

export CC:=$(CC) --sysroot=$(PWD)/sysroot
export CC:=$(shell if echo $(HOST) | grep -Eq -- '-elf($$|-)'; then \
	echo "$(CC) -isystem=$(INCLUDEDIR)" ; \
fi)

export NASMFLAGS := -felf64
export LDFLAGS :=

export PREFIX := /usr
export EXEC_PREFIX := $(EXEC_PREFIX)/boot
export BOOTDIR := /boot
export DESTDIR := $(PWD)/sysroot

export KERN ?= chronos

export KERNEL ?= $(KERN).kern
export ISO ?= $(KERN).iso

export TARGET := $(HOSTARCH)-$(KERN)

PROJECTS := kernel

EMUARGS = -sdl -no-frame -k en-us
EMUARGS += -m 1024
EMUARGS += -serial stdio
EMUARGS += -M accel=kvm:tcg
EMUARGS += -cdrom $(ISO)

EMU := qemu-system-$(ARCH)

.PHONY: all qemu kernel clean $(ISO)

all: kernel $(ISO)
	@ echo "Building ChronOS"

$(ISO):
	@ echo "Creating ISO"
	@ mkdir -p isodir
	@ mkdir -p isodir/boot
	@ mkdir -p isodir/boot/grub
	@ cp util/grub/grub.cfg isodir/boot/grub/grub.cfg
	@ cp $(DESTDIR)$(BOOTDIR)/$(KERNEL) isodir/$(BOOTDIR)/$(KERNEL)
	@ grub-mkrescue -o $@ isodir > /dev/null 2>&1
	@ echo "Done"

kernel:
	@ echo "Building $@"
	@ for PROJECT in $(PROJECTS); do \
		echo "    $$PROJECT"; \
		$(MAKE) -s -C $$PROJECT install; \
	done
	@ echo "Done" 

qemu: all
	@ echo "Starting QEMU"
	$(EMU) $(EMUARGS)

clean:
	@ echo "Cleaning"
	@ for PROJECT in $(PROJECTS); do \
		echo "    $$PROJECT"; \
		$(MAKE) -s -C $$PROJECT clean; \
	done
	@ echo "    Root project"
	@ -rm -rf sysroot
	@ -rm -rf isodir
	@ -rm -rf $(ISO)
	@ echo "Done"

