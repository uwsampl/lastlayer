# LastLayer

LastLayer is a tool for integrating hardware and software early in the design process, enabling
simulation at the system-level in a cycle-accurate fashion. This means that designs expressed in
Verilog can be interfaced and simulated with any other any other programming language that supports
a C foreing function interface (FFI). LastLayer provides a mechanism to access with registers and
memories in the design according to user specification. This mechanism is based on automatically
generating a wrapper interface for these resources in a similar fashion to what [SWIG](http://www.swig.org/)
does for C and C++ programs. Moreover, there is no special requirements on how close to completion
the hardware design is for integrating it with other programming languages. One could integrate
a desgin based on a simple logical gate or fully fledge hardware design such as a processor
using this tool. Concretely, the tool provides the following features:

* Automatic wrapper interface generation, which generates Direct Programming Interface (DPI) functions
for writing and reading registers and memories according to user specification.

* Extensible build system, which compiles the Verilog design together with the generated interface into
C++ with [Verilator](https://www.veripool.org/wiki/verilator). Then, it packages the design into
a shared library that can be easily loaded in other languages.

* Flexible device interface, which exposes a fixed interface to interact with the shared library generated
by the build system together with functions for controlling hardware simulation.

We evaluated two representative system integration design patterns: a hardware integrated with
NumPy and a ReLu accelerator integrated with PyTorch using TorchScript.

Finally, the tool was developed in Rust and tested on the following version Rust version `1.4.0`.

## Build steps

The tool will build Verilator from source and install PyTorch and NumPy with Miniconda check `build.rs`

* Install rust `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
* Install `flex bison autoconf g++ make git sbt`
    * `sbt` is only needed for the ReLu/PyTorch example, because ReLu is designed in Chisel
* Build everything `cargo build`. This will download and build Verilator and miniconda see `build.rs`

## Run examples

* Verilog Adder with NumPy `cargo run --example adder`
* Chisel Relu with PyTorch `cargo run --example relu`
