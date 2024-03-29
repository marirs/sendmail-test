#!/bin/bash

# apple silicon binary
cargo b --release --target aarch64-apple-darwin
# apple intel binary
cargo b --release --target x86_64-apple-darwin
# linux binary aarch64
cargo b --release --target aarch64-unknown-linux-gnu
# linux binary intel 64bit
cargo b --release --target x86_64-unknown-linux-gnu
# windows binary intel 64bit
cargo b --release --target x86_64-pc-windows-gnu

# remove existing files
rm -rf dist
# make the folder again
mkdir -p dist

# copy files to the dist folder
# macos
cp target/aarch64-apple-darwin/release/sendmail-test dist/sendmail-test_macos_aarch64
cp target/x86_64-apple-darwin/release/sendmail-test dist/sendmail-test_macos_x86-64
# linux
cp target/aarch64-unknown-linux-gnu/release/sendmail-test dist/sendmail-test_linux_aarch64
cp target/x86_64-unknown-linux-gnu/release/sendmail-test dist/sendmail-test_linux_x86-64
# windows
cp target/x86_64-pc-windows-gnu/release/sendmail-test.exe dist/sendmail-test_windows_x86-64.exe