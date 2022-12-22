#!/bin/bash
set -eo pipefail

PACKAGE_NAME=lychee

cd /lychee
git pull --force

#amd64
cargo deb --package ${PACKAGE_NAME}

#arm64
rustup target add arm-unknown-linux-gnueabihf
cargo deb --package ${PACKAGE_NAME} --target=arm-unknown-linux-gnueabihf

#i386
rustup target add i686-unknown-linux-gnu
cargo deb --package ${PACKAGE_NAME} --target=i686-unknown-linux-gnu

find target/ -name \*.deb -exec cp {} /debs \;