RUST_SRC = $(wildcard src/**/*.rs src/*.rs)

all: run-tests rusp

librusp.rlib: $(RUST_SRC)
	rustc src/lib.rs -o librusp.rlib

run-tests: $(RUST_SRC)
	rustc --test src/lib.rs -o run-tests

rusp: $(RUST_SRC) librusp.rlib
	rustc src/main.rs -L. -o rusp
