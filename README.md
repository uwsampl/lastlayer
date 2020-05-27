# LastLayer

LastLayer is an open-source tool that enables hardware and software continuous integration and simulation. Compared to traditional testing approaches based on the register transfer level (RTL) abstraction, LastLayer provides a mechanism for testing Verilog designs with any programming language that supports the C foreign function interface (CFFI). Furthermore, it supports a generic C interface that allows external programs convenient access to storage resources such as registers and memories in the design as well as control over the hardware simulation. Moreover, LastLayer achieves this software integration without requiring any hardware modification and automatically generates language bindings for these storage resources according to user specification. Using LastLayer, we evaluated two representative integration examples: a hardware adder written in Verilog operating over NumPy arrays, and a ReLu vector-accelerator written in Chisel processing tensors from PyTorch.

\[[Paper](https://ieeexplore.ieee.org/document/9099634)\] \[[PDF](https://homes.cs.washington.edu/~vegaluis/pdf/ieeemicro20_vega_lastlayer.pdf)\]

## Build steps

The tool will build Verilator from source and install PyTorch and NumPy with Miniconda. See [build.rs](build.rs).

* Install rust `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
* Install `wget flex bison autoconf g++ make git sbt`
    * `sbt` is only needed for the ReLu/PyTorch example, because ReLu is designed in Chisel
* Build everything `cargo build`.

## Run examples

* Verilog Adder with NumPy `cargo run --example adder`
* Chisel Relu with PyTorch `cargo run --example relu`

## License

[Apache-2.0](LICENSE) license.
