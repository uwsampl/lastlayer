use lastlayer::util::get_manifest_dir;
use lastlayer::Build;

fn main() {
    let miniconda_torch_dir =
        get_manifest_dir().join("miniconda/local/lib/python3.7/site-packages/torch");
    let pytorch_dir = get_manifest_dir().join("pytorch");
    let build_dir = pytorch_dir.join("build");
    Build::new()
        .out_dir(&build_dir)
        .top_module("Relu")
        .cc_flag("-std=c++11")
        .cc_link_dir(&miniconda_torch_dir.join("lib"))
        .cc_include_dir(&miniconda_torch_dir.join("include"))
        .cc_link_lib("c10")
        .cc_link_lib("torch")
        .cc_link_lib("shm")
        .cc_link_lib("torch_python")
        .cc_file(&pytorch_dir.join("relu.cc"))
        .verilog_file(&pytorch_dir.join("verilog/Relu.v"))
        .verilog_file(&pytorch_dir.join("verilog/Exit.v"))
        .verilog_file(&pytorch_dir.join("relu_dpi.v"))
        .compile("relu");
}
