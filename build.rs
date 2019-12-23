use os_info;
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
        version: Option<String>,
        jobs: Option<u32>,
        root_dir: Option<PathBuf>,
        verilator_dir: Option<PathBuf>,
        build_dir: Option<PathBuf>,
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
                cmd.arg("-j").arg(&format!("{}", self.get_jobs()));
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

mod miniconda {

    use super::*;

    #[derive(Clone, Debug)]
    pub struct Build {
        root_dir: Option<PathBuf>,
        miniconda_version: Option<String>,
        miniconda_dir: Option<PathBuf>,
        miniconda_name: Option<String>,
        pytorch_version: Option<String>,
    }

    impl Build {
        fn get_miniconda_version(&self) -> String {
            match &self.miniconda_version {
                Some(s) => s.to_string(),
                None => panic!("miniconda version not defined"),
            }
        }

        fn get_miniconda_dir(&self) -> PathBuf {
            match &self.miniconda_dir {
                Some(d) => d.to_path_buf(),
                None => panic!("miniconda dir not defined"),
            }
        }

        fn get_miniconda_name(&self) -> String {
            match &self.miniconda_name {
                Some(s) => s.to_string(),
                None => panic!("miniconda name not defined"),
            }
        }

        fn get_pytorch_version(&self) -> String {
            match &self.pytorch_version {
                Some(s) => s.to_string(),
                None => panic!("pytorch version not defined"),
            }
        }

        fn create_miniconda_dir(&self) {
            if !self.get_miniconda_dir().exists() {
                let mut cmd = Command::new("mkdir");
                cmd.arg("-p").arg(self.get_miniconda_dir());
                run_cmd(&mut cmd);
            }
        }

        fn wget_miniconda_sh(&self) {
            let info = os_info::get();
            let platform = match info.os_type() {
                os_info::Type::Windows => panic!("Windows not supported because of Verilator"),
                os_info::Type::Macos => "MacOSX-x86_64".to_string(),
                _ => "Linux-x86_64".to_string(),
            };
            let dir = self.get_miniconda_dir();
            let script = format!("Miniconda{}-{}.sh", self.get_miniconda_version(), platform);
            let url = format!("https://repo.continuum.io/miniconda/{}", script);
            let mut cmd = Command::new("wget");
            cmd.arg(url).arg("-O").arg(&dir.join("miniconda.sh"));
            run_cmd(&mut cmd);
        }

        fn chmod_miniconda_sh(&self) {
            let dir = self.get_miniconda_dir();
            let mut cmd = Command::new("chmod");
            cmd.arg("+x").arg(&dir.join("miniconda.sh"));
            run_cmd(&mut cmd);
        }

        fn run_miniconda_sh(&self) {
            let dir = self.get_miniconda_dir();
            let mut cmd = Command::new(&dir.join("miniconda.sh"));
            cmd.arg("-b")
                .arg("-p")
                .arg(&dir.join(self.get_miniconda_name()));
            run_cmd(&mut cmd);
        }

        fn install_pytorch(&self) {
            let m_dir = self.get_miniconda_dir();
            let i_dir = m_dir.join(self.get_miniconda_name());
            let mut cmd = Command::new(&i_dir.join("bin/conda"));
            cmd.arg("install")
                .arg("-y")
                .arg(&format!("pytorch=={}", self.get_pytorch_version()))
                .arg("-c")
                .arg("pytorch");
            run_cmd(&mut cmd);
        }

        pub fn miniconda_version(&mut self, ver: &str) -> &mut Build {
            self.miniconda_version = Some(ver.to_string());
            self
        }

        pub fn pytorch_version(&mut self, ver: &str) -> &mut Build {
            self.pytorch_version = Some(ver.to_string());
            self
        }

        pub fn root_dir<P: AsRef<Path>>(&mut self, dir: P) -> &mut Build {
            self.root_dir = Some(dir.as_ref().to_owned());
            self
        }

        pub fn miniconda_dir<P: AsRef<Path>>(&mut self, dir: P) -> &mut Build {
            self.miniconda_dir = Some(dir.as_ref().to_owned());
            self
        }

        pub fn miniconda_name(&mut self, name: &str) -> &mut Build {
            self.miniconda_name = Some(name.to_string());
            self
        }

        pub fn new() -> Build {
            Build {
                root_dir: None,
                miniconda_version: None,
                miniconda_dir: None,
                miniconda_name: None,
                pytorch_version: None,
            }
        }

        pub fn install(&self) {
            let dir = self.get_miniconda_dir();
            self.create_miniconda_dir();
            if !&dir.join(self.get_miniconda_name()).exists() {
                self.wget_miniconda_sh();
                self.chmod_miniconda_sh();
                self.run_miniconda_sh();
                self.install_pytorch();
            }
        }
    }
}

fn main() {
    let root_dir = current_dir().unwrap();
    let verilator_dir = root_dir.join("verilator");
    let verilator_build_dir = verilator_dir.join("build");
    let miniconda_dir = root_dir.join("miniconda");
    verilator::Build::new()
        .version("4.024")
        .jobs(1)
        .root_dir(&root_dir)
        .verilator_dir(&verilator_dir)
        .build_dir(&verilator_build_dir)
        .compile();
    miniconda::Build::new()
        .root_dir(&root_dir)
        .miniconda_version("3-4.7.12.1")
        .miniconda_dir(&miniconda_dir)
        .miniconda_name("local")
        .pytorch_version("1.3.1")
        .install();
}
