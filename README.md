# LastLayer

Tested with `Rust 1.4.0`

## Build steps

The tool will build Verilator from source and install PyTorch and NumPy with Miniconda check `build.rs`

* Install rust `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
* Install `flex bison autoconf g++ make git sbt`
    * `sbt` is only needed for the ReLu/PyTorch example which relies on hardware designed in Chisel.
* Build everything `cargo build`. This will download and build Verilator and miniconda see `build.rs`.

## Run examples

* Verilog Adder with NumPy `cargo run --example adder`
* Chisel Relu with PyTorch `cargo run --example relu`