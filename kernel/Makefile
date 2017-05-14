LIBS := -nostdlib -lgcc

DESTDIR ?=
EXEC_PREFIX ?= $(PREFIX)
BOOTDIR ?= $(EXEC_PREFIX)/boot

ARCHDIR := arch/$(HOSTARCH)

include $(ARCHDIR)/make.config

LDFLAGS += $(KERN_ARCH_LDFLAGS) -n -Wl,--gc-sections

KERNEL ?= chronos.kern

OBJs := \
	$(KERN_ARCH_OBJs)
	
.PHONY: all install clean

all: $(KERNEL)

$(KERNEL): xargo $(OBJs)
	$(CC) $(LDFLAGS) -T link.ld -o $@ $(OBJs) target/$(TARGET)/debug/lib$(KERN).a $(LIBS)

%.o: %.asm
	$(NASM) $(NASMFLAGS) $< -o $@

xargo:
	$(XARGO) build --target $(TARGET)

clean:
	@ -rm -f $(KERNEL)
	@ -rm -f $(OBJs)
	@ xargo clean

install: install-kernel

install-kernel: $(KERNEL)
	mkdir -p $(DESTDIR)$(BOOTDIR)
	cp $(KERNEL) $(DESTDIR)$(BOOTDIR)
	