use std::env::{current_dir, set_current_dir};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::Command;

fn run_cmd(cmd: &mut Command) {
    println!("running {:?}", cmd);
    let output = cmd.output().expect("failed to execute process");
    println!("status: {}", output.status);
    io::stdout().write_all(&output.stdout).unwrap();
    io::stderr().write_all(&output.stderr).unwrap();
    assert!(output.status.success());
}

mod verilator {

    use super::*;

    pub struct Build {
        pub version: String,
        pub root_path: PathBuf,
        pub verilator_path: PathBuf,
        pub build_path: PathBuf,
    }

    impl Build {
        fn set_version(&self) {
            let mut cmd = Command::new("git");
            cmd.arg("-C")
                .arg(&self.verilator_path)
                .arg("checkout")
                .arg(&format!("v{}", self.version))
                .arg("-b")
                .arg(&format!("build_v{}", self.version));
            run_cmd(&mut cmd);
        }

        fn download(&self) {
            if !self.verilator_path.exists() {
                let mut cmd = Command::new("git");
                cmd.arg("clone")
                    .arg("https://git.veripool.org/git/verilator");
                run_cmd(&mut cmd);
                self.set_version();
            }
            assert!(self.verilator_path.exists());
        }

        fn change_to_verilator_dir(&self) {
            set_current_dir(&self.verilator_path).expect("failed to change dir");
        }

        fn change_to_root_dir(&self) {
            set_current_dir(&self.root_path).expect("failed to change dir");
        }

        fn autoconf(&self) {
            self.change_to_verilator_dir();
            let mut cmd = Command::new("autoconf");
            run_cmd(&mut cmd);
            self.change_to_root_dir();
        }

        fn configure(&self) {
            self.change_to_verilator_dir();
            let mut cmd = Command::new("./configure");
            cmd.arg("--prefix").arg(&self.build_path);
            run_cmd(&mut cmd);
        }

        pub fn new(version: &str, root_path: &Path, verilator_path: &Path) -> Build {
            Build {
                version: version.to_string(),
                root_path: root_path.to_path_buf(),
                verilator_path: verilator_path.to_path_buf(),
                build_path: verilator_path.join("build"),
            }
        }

        pub fn compile(&self) {
            self.download();
            self.autoconf();
            self.configure();
        }
    }
}

fn main() {
    let version = "4.024";
    let root_path = current_dir().unwrap();
    let verilator_path = root_path.join("verilator");
    let verilator = verilator::Build::new(&version, &root_path, &verilator_path);
    verilator.compile();
}
