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

fn main() {
    let verilator_path = PathBuf::from("verilator");
    get_verilator(&verilator_path);
}