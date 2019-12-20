use lastlayer::util::get_manifest_dir;
use lastlayer::Build;

fn main() {
    let out_dir = get_manifest_dir().join("test");
    let v_dir = get_manifest_dir().join("verilog");
    Build::new()
        .out_dir(&out_dir)
        .top_module("test")
        .verilog_file(&v_dir.join("test.v"))
        .compile("test");
}
