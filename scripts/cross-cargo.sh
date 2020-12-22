#! /bin/bash

source /usr/local/oecore-x86_64/environment-setup-cortexa9hf-neon-oe-linux-gnueabi

cargo watch -x "check --workspace --message-format=short"