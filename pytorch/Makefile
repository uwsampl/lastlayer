# PYTHON = $(abspath ..)/miniconda/local/bin/python3.7

# run: build/librelu.so
# 	${PYTHON} relu.py

# build/librelu.so:
# 	cargo run --bin pytorch_relu

default: verilog/Relu.v

verilog/Relu.v:
	cd chisel && sbt 'runMain relu.Relu --target-dir ../verilog'

clean:
	-rm -rf build __pycache__ verilog
