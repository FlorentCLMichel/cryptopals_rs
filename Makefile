.PHONY: test build_debug clean

test:
	cargo test --offline

build_debug:
	cargo build --offline

build_release:
	cargo build --release --offline

clean: 
	rm -f Cargo.lock
	rm -rf target
