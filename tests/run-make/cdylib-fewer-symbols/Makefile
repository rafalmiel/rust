# ignore-cross-compile

# Test that allocator-related symbols don't show up as exported from a cdylib as
# they're internal to Rust and not part of the public ABI.
# See https://github.com/rust-lang/rust/commit/fbf98697021173a30b84d9145df0966a23a2f9d2

include ../tools.mk

# ignore-windows
# FIXME: The __rdl_ and __rust_ symbol still remains, no matter using MSVC or GNU
# See https://github.com/rust-lang/rust/pull/46207#issuecomment-347561753

all:
	$(RUSTC) foo.rs
	nm -g "$(call DYLIB,foo)" | $(CGREP) -v __rdl_ __rde_ __rg_ __rust_
