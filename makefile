all: build deploy

build:
	trunk build --release

deploy:
	neocities push --prune dist/
