#!/bin/bash
set -eo pipefail

PACKAGE_NAME=lychee

cd /lychee
git pull --force

cargo deb --package ${PACKAGE_NAME}
find target/ -name \*.deb -exec cp {} /debs \;