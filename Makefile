logginglib1/target/debug/liblogginglib1.a:
	RUSTFLAGS="-C metadata=something_unique" cargo build -v --manifest-path logginglib1/Cargo.toml

logginglib2/target/debug/liblogginglib2.a:
	RUSTFLAGS="-C metadata=more_unique" cargo build -v --manifest-path logginglib2/Cargo.toml

main: logginglib1/target/debug/liblogginglib1.a logginglib2/target/debug/liblogginglib2.a
	cc -o main main.c -L logginglib2/target/debug/ -llogginglib2 -L logginglib1/target/debug/ -llogginglib1 -lpthread -ldl

test: main
	./main

clean:
	cargo clean --manifest-path logginglib1/Cargo.toml
	cargo clean --manifest-path logginglib2/Cargo.toml
	rm -f ./main
