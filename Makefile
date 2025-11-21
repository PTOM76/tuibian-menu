all: build deb

build:
	cargo build --release

deb: deb-x86_64 deb-aarch64

exec:
	cargo run

install:
	sudo dpkg -i target/debian/*.deb

deb-x86_64:
	cargo deb --target x86_64-unknown-linux-gnu

deb-aarch64:
	cargo deb --target aarch64-unknown-linux-gnu
