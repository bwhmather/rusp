RUST_SRC = $(wildcard src/**/*.rs src/*.rs)
LIBRUSP = $(shell rustc --crate-file-name src/lib.rs)

all: run-tests rusp

$(LIBRUSP): $(RUST_SRC)
	rustc src/lib.rs

run-tests: $(RUST_SRC)
	rustc --test src/lib.rs -o run-tests

rusp: $(RUST_SRC) $(LIBRUSP)
	rustc src/main.rs -L. -o rusp
