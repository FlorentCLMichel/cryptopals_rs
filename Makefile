.PHONY: test build_debug build_release clippy clean

test:
	cargo test --offline

build_debug:
	cargo build --offline

build_release:
	cargo build --release --offline

clippy:
	cargo clippy --offline

clean: 
	rm -f Cargo.lock
	rm -rf target
