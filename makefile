all: test

test: 
	trunk serve --open

build:
	trunk build --release

deploy: build
	neocities push --prune dist/

