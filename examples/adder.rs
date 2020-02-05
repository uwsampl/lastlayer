use lastlayer::util::{get_lastlayer_root_dir, run_cmd};
use lastlayer::Build;
use std::path::Path;
use std::process::Command;

fn lastlayer_build(build_dir: &Path, adder_dir: &Path) {
    Build::new()
        .out_dir(build_dir)
        .top_module("adder")
        .verilog_file(&adder_dir.join("adder.v"))
        .add_register(0, "adder.a", 8)
        .add_register(1, "adder.b", 8)
        .add_register(2, "adder.y", 8)
        .compile("adder");
}

fn run_test(bin: &Path, adder_dir: &Path) {
    let mut cmd = Command::new(bin);
    cmd.arg(adder_dir.join("test.py"));
    run_cmd(&mut cmd);
}

fn main() {
    let adder_dir = get_lastlayer_root_dir().join("examples/adder");
    let build_dir = &adder_dir.join("build");
    let python_bin = get_lastlayer_root_dir().join("miniconda/local/bin/python3.7");
    lastlayer_build(&build_dir, &adder_dir);
    run_test(&python_bin, &adder_dir);
}
