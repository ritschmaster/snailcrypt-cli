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
	rm -f snailcrypt-gtk_.zip
	zip snailcrypt-cli_.zip LICENSE README target/release/snailcrypt-cli
