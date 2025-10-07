all: test

test: 
	trunk serve --open

clean:
	trunk clean

build: clean
	trunk build --release

deploy: build
	neocities push --prune dist/

