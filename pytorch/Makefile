PYTHON = $(abspath ..)/miniconda/local/bin/python3.7

run: build/librelu.so
	${PYTHON} relu.py

build/librelu.so: verilog/Relu.v
	cargo run --bin pytorch_relu

verilog/Relu.v:
	cd chisel && sbt 'runMain relu.Relu --target-dir ../verilog'

clean:
	-rm -rf build __pycache__ verilog
