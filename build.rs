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

mod verilator {
    use super::*;
    fn download(path: &Path) {
        if !path.exists() {
            let mut cmd = Command::new("git");
            cmd.arg("clone")
                .arg("https://git.veripool.org/git/verilator");
            run_cmd(&mut cmd);
        }
        assert!(path.exists());
    }

    fn set_version(path: &Path, ver: &str) {
        let mut cmd = Command::new("git");
        cmd.arg("-C")
            .arg(path)
            .arg("checkout")
            .arg(&format!("v{}", ver));
        run_cmd(&mut cmd);
    }

    pub fn build() {
        let version = "4.024";
        let build_path = PathBuf::from("verilator");
        download(&build_path);
        set_version(&build_path, version);
    }
}


fn main() {
    verilator::build();
}