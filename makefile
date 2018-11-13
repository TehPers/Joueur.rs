# Please do not change this file - the build may fail if you do

all: core

core:
	cargo build --release

clean:
@rm -rf build