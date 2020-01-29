all:
	make dependencies
	make core

dependencies: ;

core:
	cargo build

clean:
	cargo clean
	rm -rf target
