RUST_SRC = src/parse.rs

LIBRUSP = $(shell rustc --crate-file-name src/lib.rs)

all: run-tests rusp

$(LIBRUSP): src/lib.rs $(RUST_SRC)
	rustc $<

run-tests: src/lib.rs $(RUST_SRC)
	rustc --test $< -o run-tests

rusp: src/main.rs $(LIBRUSP)
	rustc $< -L. -o rusp
