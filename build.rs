use std::path::{Path, PathBuf};
use std::process::Command;

fn run_cmd(cmd: &mut Command) {
    let status = match cmd.status() {
        Ok(status) => status,
        Err(_) => panic!("failed to execute command"),
    };
    if !status.success() {
        panic!("command did not execute successfully");
    }
}

fn get_verilator(path: &Path) {
    if !path.exists() {
        let mut cmd = Command::new("git");
        cmd.arg("clone")
            .arg("https://git.veripool.org/git/verilator");
        run_cmd(&mut cmd);
    }
}

fn set_verilator_version(path: &Path, ver: &str) {
    if path.exists() {
        let mut cmd = Command::new("git");
        cmd.arg("-C")
            .arg(path)
            .arg("checkout")
            .arg(&format!("v{}", ver));
        run_cmd(&mut cmd);
    }
}

fn main() {
    let version = "4.024";
    let build_path = PathBuf::from("verilator");
    get_verilator(&build_path);
    set_verilator_version(&build_path, version);
}