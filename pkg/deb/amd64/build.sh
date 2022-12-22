#!/bin/bash

# This is a shell script because Makefiles run in a different shell
# and Docker for Mac won't allow file mounts.

set -eo pipefail

docker build -t lychee-deb-builder .
docker run --init -v "$(pwd)/debs:/debs" lychee-deb-builder