all:
	make dependencies
	make core

dependencies: ;

core:
	cargo build

clean:
	rm -rf target
