run: telegram

telegram:
	cargo build --bin=telegram
	./target/debug/telegram

check: test
	
test:
	cargo test
