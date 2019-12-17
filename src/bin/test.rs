use lastlayer::util::get_manifest_dir;

fn main() {
    let verilator = verilator::Build::new()
    .out_dir(get_manifest_dir().join("test"));
    println!("{:?}", verilator.compile_verilog());
}