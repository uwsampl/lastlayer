use lastlayer::util::{get_manifest_dir, run_cmd};
use lastlayer::Build;
use std::path::Path;
use std::process::Command;

fn build(build_dir: &Path, adder_dir: &Path) {
    Build::new()
        .out_dir(build_dir)
        .top_module("adder")
        .verilog_file(&adder_dir.join("adder.v"))
        .verilog_file(&adder_dir.join("adder_dpi.v"))
        .compile("adder");
}

fn run_test(bin: &Path, adder_dir: &Path) {
    let mut cmd = Command::new(bin);
    cmd.arg(adder_dir.join("test.py"))
        .arg("--adder-dir")
        .arg(adder_dir);
    run_cmd(&mut cmd);
}

fn main() {
    let adder_dir = get_manifest_dir().join("examples/adder");
    let build_dir = &adder_dir.join("build");
    let python_bin = get_manifest_dir().join("miniconda/local/bin/python3.7");
    build(&build_dir, &adder_dir);
    run_test(&python_bin, &adder_dir);
}