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

    #[derive(Clone, Debug)]
    pub struct Build {
        pub version: Option<String>,
        pub jobs: Option<u32>,
        pub root_dir: Option<PathBuf>,
        pub verilator_dir: Option<PathBuf>,
        pub build_dir: Option<PathBuf>,
    }

    impl Build {

        fn get_root_dir(&self) -> PathBuf {
            match &self.root_dir {
                Some(d) => d.to_path_buf(),
                None => panic!("root dir not defined"),
            }
        }

        fn get_verilator_dir(&self) -> PathBuf {
            match &self.verilator_dir {
                Some(d) => d.to_path_buf(),
                None => panic!("verilator dir not defined"),
            }
        }

        fn get_build_dir(&self) -> PathBuf {
            match &self.build_dir {
                Some(d) => d.to_path_buf(),
                None => panic!("build dir not defined"),
            }
        }

        fn get_version(&self) -> String {
            match &self.version {
                Some(s) => s.to_string(),
                None => panic!("version not defined"),
            }
        }

        fn get_jobs(&self) -> u32 {
            match &self.jobs {
                Some(n) => *n,
                None => panic!("jobs not defined"),
            }
        }

        fn get_verilator_bin(&self) -> PathBuf {
            self.get_build_dir().join("bin/verilator")
        }

        fn set_branch(&self) {
            let mut cmd = Command::new("git");
            cmd.arg("-C")
                .arg(self.get_verilator_dir())
                .arg("checkout")
                .arg(&format!("v{}", self.get_version()))
                .arg("-b")
                .arg(&format!("build_v{}", self.get_version()));
            run_cmd(&mut cmd);
        }

        fn download(&self) {
            if !self.get_verilator_dir().exists() {
                let mut cmd = Command::new("git");
                cmd.arg("clone")
                    .arg("https://git.veripool.org/git/verilator");
                run_cmd(&mut cmd);
                self.set_branch();
            }
            assert!(self.get_verilator_dir().exists());
        }

        fn cd_to_verilator_dir(&self) {
            set_current_dir(self.get_verilator_dir()).expect("failed to change dir");
            println!("cd to {:?}", current_dir().unwrap());
        }

        fn cd_to_root_dir(&self) {
            set_current_dir(self.get_root_dir()).expect("failed to change dir");
            println!("cd to {:?}", current_dir().unwrap());
        }

        fn autoconf(&self) {
            self.cd_to_verilator_dir();
            let mut cmd = Command::new("autoconf");
            run_cmd(&mut cmd);
            self.cd_to_root_dir();
        }

        fn configure(&self) {
            self.cd_to_verilator_dir();
            let mut cmd = Command::new("./configure");
            cmd.arg("--prefix").arg(self.get_build_dir());
            run_cmd(&mut cmd);
            self.cd_to_root_dir();
        }

        fn make(&self) {
            self.cd_to_verilator_dir();
            let mut cmd = Command::new("make");
            if self.get_jobs() > 0 {
                cmd.arg("-j")
                    .arg(&format!("{}", self.get_jobs()));
            }
            run_cmd(&mut cmd);
            self.cd_to_root_dir();
        }

        fn install(&self) {
            self.cd_to_verilator_dir();
            let mut cmd = Command::new("make");
            cmd.arg("install");
            run_cmd(&mut cmd);
            self.cd_to_root_dir();
        }

        pub fn new() -> Build {
            Build {
                version: None,
                jobs: None,
                root_dir: None,
                verilator_dir: None,
                build_dir: None,
            }
        }

        pub fn version(&mut self, ver: &str) -> &mut Build {
            self.version = Some(ver.to_string());
            self
        }

        pub fn jobs(&mut self, n: u32) -> &mut Build {
            self.jobs = Some(n);
            self
        }

        pub fn root_dir<P: AsRef<Path>>(&mut self, dir: P) -> &mut Build {
            self.root_dir = Some(dir.as_ref().to_owned());
            self
        }

        pub fn verilator_dir<P: AsRef<Path>>(&mut self, dir: P) -> &mut Build {
            self.verilator_dir = Some(dir.as_ref().to_owned());
            self
        }

        pub fn build_dir<P: AsRef<Path>>(&mut self, dir: P) -> &mut Build {
            self.build_dir = Some(dir.as_ref().to_owned());
            self
        }

        pub fn compile(&self) {
            if !self.get_verilator_bin().exists() {
                self.download();
                self.autoconf();
                self.configure();
                self.make();
                self.install();
            }
        }
    }
}

fn main() {
    let root_dir = current_dir().unwrap();
    let verilator_dir = root_dir.join("verilator");
    let build_dir = verilator_dir.join("build");
    verilator::Build::new()
        .version(&"4.024")
        .jobs(1)
        .root_dir(&root_dir)
        .verilator_dir(&verilator_dir)
        .build_dir(&build_dir)
        .compile();
}
