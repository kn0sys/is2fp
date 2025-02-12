#!/bin/bash
# Reproducible build is2fp release
# Run from is2fp root
# usage: ./scripts/build_release vX.X.X-ver

# Linux x86_64 output directory
LINUX_X86_64="x86_64-linux-gnu"
RELEASE_NAME="is2fp-$LINUX_X86_64-$1"
LINUX_OUTPUT_DIR=".build/release/$RELEASE_NAME"
mkdir -p $LINUX_OUTPUT_DIR
# build jars for j4-i2p-rs
git clone --depth 1 https://github.com/kn0sys/i2p.i2p
cd i2p.i2p && ant buildRouter buildI2PTunnelJars buildSAM jbigi buildAddressbook
mkdir -p ../opt/j4-i2p-rs/jassets && cp build/* ../opt/j4-i2p-rs/jassets/
cd ../
# certificates for i2p reseed
cp -r j4-i2p-rs/certificates $LINUX_OUTPUT_DIR
# build is2fp
cargo build --release
# j4-i2p-rs dependencies
cp -r j4-i2p-rs/opt/j4-i2p-rs/deps opt/j4-i2p-rs
cp j4-i2p-rs/opt/j4-i2p-rs/jassets/j4rs-0.22.0-jar-with-dependencies.jar opt/j4-i2p-rs/jassets
cp -r opt/ $LINUX_OUTPUT_DIR
cp target/release/is2fp $LINUX_OUTPUT_DIR
# make the bzip for linux
cd .build/release/ && tar -cjf $RELEASE_NAME.tar.bz2 $RELEASE_NAME/ && mv $RELEASE_NAME.tar.bz2 ../../
