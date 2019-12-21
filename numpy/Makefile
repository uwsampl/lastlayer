PYTHON = $(abspath ..)/miniconda/local/bin/python3.7

run: build/libadder.so
	${PYTHON} adder.py

build/libadder.so:
	cargo run --bin numpy_adder

clean:
	-rm -rf build __pycache__
