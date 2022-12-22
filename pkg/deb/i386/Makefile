.PHONY: build
build:
	docker build -t lychee-deb-builder .
	docker run --init -v "$(pwd)/debs:/debs" lychee-deb-builder