PYTHON = $(abspath ..)/miniconda/local/bin/python3.7

run: build/librelu.so
	${PYTHON} relu.py

build/librelu.so:
	cargo run --bin pytorch_relu

clean:
	-rm -rf build __pycache__
