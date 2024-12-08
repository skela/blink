mkfile_path = $(abspath $(lastword $(MAKEFILE_LIST)))
mkfile_dir = $(dir $(mkfile_path))

build:
	cargo build --release

run: build
	target/release/blink samples/ --output results/

run_1: build
	target/release/blink samples/test1.dart --output results/

run_new: build
	target/release/blink samples/ --output results/ -n

all:
	docker build --platform darwin/arm64 . -t blink-macos-arm64
	docker build --platform linux/amd64 . -t blink-linux-amd64
	docker run -v $(mkfile_dir)/target/release/platforms/macos:/app/blink-macos blink-macos-arm64:latest cp /app/target/release/blink /app/blink-macos
	docker run -v $(mkfile_dir)/target/release/platforms/linux:/app/blink-linux blink-linux-amd64:latest cp /app/target/release/blink /app/blink-linux
