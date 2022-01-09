rose:
	cargo build --release
	strip target/release/rose

install:
	cargo install --path .
	strip ~/.cargo/bin/rose
