-include ../tools.mk

SKIP_OS := 'FreeBSD OpenBSD Bitrig SunOS'

ifneq ($(UNAME),$(findstring $(UNAME),$(SKIP_OS)))

TARGET := $(shell $(RUSTC) -vV | grep 'host:' | sed 's/host: //')

all:
	$(RUSTC) foo.rs -C extra-filename=-host
	$(RUSTC) bar.rs -C extra-filename=-targ --target $(TARGET)
	$(RUSTC) baz.rs --extern a=$(TMPDIR)/liba-targ.rlib --target $(TARGET)
else
# FreeBSD, OpenBSD, and Bitrig support only x86_64 architecture for now
all:
endif
