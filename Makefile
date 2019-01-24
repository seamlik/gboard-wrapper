.PHONY: all clean deb rust

all: deb

deb:
	dpkg-buildpackage

rust:
	cargo build --release

clean:
	cargo clean
	git clean --force -X -d