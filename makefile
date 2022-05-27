all: build
	cargo run --release

art: build
	cargo run --release art

build:
	cargo build --release

clean: clean_anim
	cargo clean

clean_anim:
	rm animation/*.ppm