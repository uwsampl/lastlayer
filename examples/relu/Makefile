#PYTHON = $(abspath ..)/miniconda/local/bin/python3.7
#
#run: build/librelu.so
#	${PYTHON} relu.py
#
#build/librelu.so: verilog/Relu.v
#	cargo run --bin pytorch_relu

chisel: verilog/Relu.v
verilog/Relu.v:
	cd chisel && sbt " runMain relu.Relu --target-dir ../v2 --num-vector-words 2 "

clean:
	-rm -rf build __pycache__ verilog
