all: test

test: 
	trunk serve --open

clean:
	trunk clean
	cargo clean

build: clean
	trunk build --release

deploy: build
	neocities push --prune dist/

