use lastlayer::util::get_manifest_dir;
use lastlayer::Build;

fn main() {
    let numpy_dir = get_manifest_dir().join("numpy");
    let build_dir = numpy_dir.join("build");
    Build::new()
        .out_dir(&build_dir)
        .top_module("adder")
        .verilog_file(&numpy_dir.join("adder.v"))
        .verilog_file(&numpy_dir.join("adder_dpi.v"))
        .compile("adder");
}
