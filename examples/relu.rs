use lastlayer::util::{change_dir, get_manifest_dir, run_cmd};
use lastlayer::Build;
use std::path::Path;
use std::process::Command;

fn compile_chisel(num_vec_words: u32) {
    let manifest_dir = get_manifest_dir();
    let chisel_dir = &manifest_dir.join("examples/relu/chisel");
    let sbt_opt = format!(
        "runMain relu.Relu --target-dir ../relu_{} --num-vec-words {}",
        num_vec_words, num_vec_words
    );
    change_dir(&chisel_dir);
    let mut cmd = Command::new("sbt");
    cmd.arg(&sbt_opt);
    run_cmd(&mut cmd);
    change_dir(&manifest_dir);
}

fn lastlayer_build(torch_dir: &Path, relu_dir: &Path, num_vec_words: u32) {
    let mem_width = num_vec_words * 4 * 8;
    Build::new()
        .out_dir(relu_dir.join(format!("relu_{}", num_vec_words)))
        .top_module("Relu")
        .cc_flag("-std=c++11")
        .cc_link_dir(torch_dir.join("lib"))
        .cc_include_dir(torch_dir.join("include"))
        .cc_link_lib("c10")
        .cc_link_lib("torch")
        .cc_link_lib("shm")
        .cc_link_lib("torch_python")
        .cc_file(relu_dir.join("relu.cc"))
        .verilog_file(relu_dir.join(format!("relu_{}/Relu.v", num_vec_words)))
        .verilog_file(relu_dir.join(format!("relu_{}/Exit.v", num_vec_words)))
        .add_register(0, "Relu.raddr", 16)
        .add_register(1, "Relu.waddr", 16)
        .add_register(2, "Relu.launch", 1)
        .add_register(3, "Relu.finish", 1)
        .add_register(4, "Relu.length", 16)
        .add_register(5, "Relu.cycle", 32)
        .add_memory(0, "Relu.rmem", mem_width.clone())
        .add_memory(1, "Relu.wmem", mem_width.clone())
        .compile(&format!("relu_{}", num_vec_words));
}

fn run_test(bin: &Path, relu_dir: &Path, num_vec_words: u32) {
    let mut cmd = Command::new(bin);
    cmd.arg(relu_dir.join("relu.py"))
        .arg("--num-vec-words")
        .arg(&format!("{}", num_vec_words));
    run_cmd(&mut cmd);
}

fn main() {
    let torch_dir = get_manifest_dir().join("miniconda/local/lib/python3.7/site-packages/torch");
    let python_bin = get_manifest_dir().join("miniconda/local/bin/python3.7");
    let relu_dir = get_manifest_dir().join("examples/relu");
    let total = 1;
    let base: u32 = 2;
    let repeat = 16;
    for i in 0..total {
        compile_chisel(base.pow(i));
        lastlayer_build(&torch_dir, &relu_dir, base.pow(i));
        for _i in 0..repeat {
            run_test(&python_bin, &relu_dir, base.pow(i));
        }
    }
}
