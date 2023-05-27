SYSTEM_NAME=$(shell uname -o)
ARCH_NAME=$(shell uname -m)

PROJECT_NAME=$(shell cargo pkgid | cut -d\# -f1 | xargs basename)
VERSION_NAME=$(shell cargo pkgid | cut -d\# -f2)
DIST_NAME=snailcrypt-cli_$(VERSION_NAME)_$(SYSTEM_NAME)_$(ARCH_NAME)

all:
	cargo auditable build --release

debug:
	cargo build
	
release:
	cargo build --release
	
check:
	cargo test

audit:
	cargo audit bin target/release/snailcrypt-cli
	
dist: release
	rm -f $(DIST_NAME).zip
	zip --junk-paths $(DIST_NAME).zip LICENSE.txt README.txt target/release/snailcrypt-cli
