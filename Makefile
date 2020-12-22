SHELL := /bin/bash
TARGET := ./target/armv7-unknown-linux-gnueabihf/release/counter
SRC := ./src/**.rs

.PHONY: build install uninstall run clean


$(TARGET): $(SRC)
	source /usr/local/oecore-x86_64/environment-setup-cortexa9hf-neon-oe-linux-gnueabi && \
	cargo +nightly build --release --target=armv7-unknown-linux-gnueabihf # --features "enable-runtime-benchmarking"

build: $(TARGET)

clean:
	rm $(TARGET)

.installed: $(TARGET)
	- ssh rm killall counter
	scp $(TARGET) rm:
	touch .installed
install: .installed

uninstall:
	ssh rm rm ./counter
	rm .installed

run: install
	./run-on-device.sh