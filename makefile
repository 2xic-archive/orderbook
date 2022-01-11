
.PHONY: gym

gym:
	cd orderbook && cargo build --release
	cp ./orderbook/target/release/liborderbooklib.so gym/liborderbooklib.so
	cd gym && python3 test.py

test:
	cd orderbook && cargo test