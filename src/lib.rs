use crate::util::{get_lastlayer_root_dir, run_cmd};
use handlebars::Handlebars;
use serde::Serialize;
use std::error::Error;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::process::Command;

pub mod awig;
pub mod util;

#[derive(Clone, Debug)]
pub struct Register {
    pub hid: u32,
    pub path: String,
    pub width: u32,
}

#[derive(Clone, Debug)]
pub struct Memory {
    pub hid: u32,
    pub path: String,
    pub width: u32,
}

pub struct Build {
    tool_name: String,
    virtual_top_name: Option<String>,
    top_name: Option<String>,
    clock_name: Option<String>,
    reset_name: Option<String>,
    verilog_warnings: Vec<String>,
    verilog_files: Vec<PathBuf>,
    verilog_include_dirs: Vec<PathBuf>,
    cc_flags: Vec<String>,
    cc_include_dirs: Vec<PathBuf>,
    cc_link_dirs: Vec<PathBuf>,
    cc_link_libs: Vec<String>,
    cc_files: Vec<PathBuf>,
    out_dir: Option<PathBuf>,
    handlebars_dir: Option<PathBuf>,
    bin: Option<PathBuf>,
    reg: Vec<Register>,
    mem: Vec<Memory>,
}

#[derive(Serialize)]
struct VirtualHandle {
    vtop: String,
    top: String,
    clock: String,
    reset: String,
}

impl Build {
    fn get_top_name(&self) -> String {
        match self.top_name.clone() {
            Some(p) => p,
            None => panic!("Top module name not set"),
        }
    }

    fn get_virtual_top_name(&self) -> String {
        match self.virtual_top_name.clone() {
            Some(p) => p,
            None => panic!("Virtual top module name not set"),
        }
    }

    fn get_clock(&self) -> String {
        match self.clock_name.clone() {
            Some(p) => p,
            None => panic!("Clock name not set"),
        }
    }

    fn get_reset(&self) -> String {
        match self.reset_name.clone() {
            Some(p) => p,
            None => panic!("Reset name not set"),
        }
    }

    fn get_out_dir(&self) -> PathBuf {
        match &self.out_dir {
            Some(d) => d.to_path_buf(),
            None => panic!("out dir not defined"),
        }
    }

    fn get_handlebars_dir(&self) -> PathBuf {
        match &self.handlebars_dir {
            Some(d) => d.to_path_buf(),
            None => panic!("handlebars dir not defined"),
        }
    }

    fn get_bin(&self) -> PathBuf {
        match &self.bin {
            Some(b) => b.to_path_buf(),
            None => panic!("binary path not defined"),
        }
    }

    fn render(&self, input: &str, output: &str) -> Result<(), Box<dyn Error>> {
        let reg = Handlebars::new();
        let handle = VirtualHandle {
            vtop: self.get_virtual_top_name(),
            top: self.get_top_name(),
            clock: self.get_clock(),
            reset: self.get_reset(),
        };
        let template_path = self.get_handlebars_dir().join(input);
        let output_path = self.get_out_dir().join(output);
        let mut template_file = File::open(template_path)?;
        let mut output_file = File::create(output_path)?;
        reg.render_template_source_to_write(&mut template_file, &handle, &mut output_file)?;
        Ok(())
    }

    fn create_out_dir(&self) {
        let mut cmd = Command::new("mkdir");
        cmd.arg("-p").arg(self.get_out_dir());
        run_cmd(&mut cmd);
    }

    fn create_virtual_verilog_top(&mut self) -> &mut Build {
        let v_name = format!("{}.v", self.tool_name);
        let v_hbs = format!("{}.hbs", &v_name);
        let v_file = self.get_out_dir().join(&v_name);
        self.render(&v_hbs, &v_name)
            .expect("failed to render virtual top");
        self.verilog_file(&v_file);
        self
    }

    fn create_virtual_cc_top(&mut self) -> &mut Build {
        let cc_name = format!("{}.cc", self.tool_name);
        let hbs_name = format!("{}.hbs", &cc_name);
        let cc_file = self.get_out_dir().join(&cc_name);
        self.render(&hbs_name, &cc_name)
            .expect("failed to render virtual top");
        self.verilog_file(&cc_file);
        self
    }

    fn compile_awig(&mut self) -> &mut Build {
        let name = format!("{}_dpi", self.get_virtual_top_name());
        let filename = format!("{}.v", &name);
        let file = self.get_out_dir().join(&filename);
        awig::compile(
            &file,
            &self.get_virtual_top_name(),
            &name,
            "dpi_reg",
            "dpi_mem",
            &self.reg,
            &self.mem,
        )
        .expect("AWIG failed");
        self.verilog_file(file);
        self
    }

    fn compile_verilog(&self) {
        let mut cmd = Command::new(self.get_bin());
        cmd.arg("--cc")
            .arg("-Mdir")
            .arg(self.get_out_dir())
            .arg("--top-module")
            .arg(self.get_virtual_top_name())
            .arg("--assert");
        for dir in self.verilog_include_dirs.iter() {
            let dir_name = dir.to_str().unwrap().to_owned();
            cmd.arg(format!("-I{}", dir_name));  // It seems that Verilator does not support space between the path and -I
        }
        for file in self.verilog_files.iter() {
            cmd.arg(file);
        }
        for warn in &self.verilog_warnings {
            cmd.arg(format!("-Wno-{}", warn));
        }
        run_cmd(&mut cmd);
    }

    fn default_verilog_warning(&mut self) -> &mut Build {
        self.verilog_disable_warning("BLKANDNBLK");
        self.verilog_disable_warning("PINMISSING");
        self.verilog_disable_warning("STMTDLY");
        self.verilog_disable_warning("WIDTH");
        self
    }

    fn default_cc_files(&mut self) -> &mut Build {
        let include_dir = get_lastlayer_root_dir().join("verilator/build/share/verilator/include");
        let out_dir = self.get_out_dir();
        self.cc_file(&include_dir.join("verilated.cpp"));
        self.cc_file(&include_dir.join("verilated_dpi.cpp"));
        self.cc_file(&out_dir.join(format!("{}.cc", self.tool_name)));
        self.cc_file(&out_dir.join(format!("V{}.cpp", self.get_virtual_top_name())));
        self.cc_file(&out_dir.join(format!("V{}__Syms.cpp", self.get_virtual_top_name())));
        self.cc_file(&out_dir.join(format!("V{}__Dpi.cpp", self.get_virtual_top_name())));
        self
    }

    fn default_include_dirs(&mut self) -> &mut Build {
        let include_dir = get_lastlayer_root_dir().join("include/lastlayer");
        let verilator_dir = get_lastlayer_root_dir().join("verilator/build/share/verilator/include");
        self.cc_include_dir(self.get_out_dir());
        self.cc_include_dir(&verilator_dir);
        self.cc_include_dir(&verilator_dir.join("vltstd"));
        self.cc_include_dir(&include_dir);
        self
    }

    fn compile_cxx(&mut self, name: &str) {
        let out_dir = self.get_out_dir();
        let mut cmd = Command::new("g++");
        cmd.arg("-shared")
            .arg("-faligned-new")
            .arg("-shared")
            .arg("-fPIC");
        for dir in self.cc_include_dirs.iter() {
            cmd.arg("-I").arg(dir);
        }
        for dir in self.cc_link_dirs.iter() {
            cmd.arg("-L").arg(dir);
        }
        for lib in self.cc_link_libs.iter() {
            cmd.arg(format!("-l{}", lib));
        }
        for flag in self.cc_flags.iter() {
            cmd.arg(flag);
        }
        for file in self.cc_files.iter() {
            cmd.arg(file);
        }
        cmd.arg("-o").arg(&out_dir.join(format!("lib{}.so", name)));
        run_cmd(&mut cmd);
    }

    fn copy_header(&self) {
        let mut cmd = Command::new("cp");
        cmd.arg(get_lastlayer_root_dir().join("include/lastlayer/lastlayer.h"))
            .arg(self.get_out_dir());
        run_cmd(&mut cmd);
    }

    fn create_link_to_verilator_include(&self) {
        let mut cmd = Command::new("ln");
        cmd.arg("-sf")
            .arg(get_lastlayer_root_dir().join("verilator/build/share/verilator/include"))
            .arg(self.get_out_dir().join("verilator"));
        run_cmd(&mut cmd);
    }

    pub fn new() -> Build {
        Build {
            tool_name: "lastlayer".to_string(),
            virtual_top_name: None,
            top_name: None,
            clock_name: Some("clock".to_string()),
            reset_name: Some("reset".to_string()),
            verilog_warnings: Vec::new(),
            verilog_files: Vec::new(),
            verilog_include_dirs: Vec::new(),
            cc_flags: Vec::new(),
            cc_include_dirs: Vec::new(),
            cc_link_dirs: Vec::new(),
            cc_link_libs: Vec::new(),
            cc_files: Vec::new(),
            out_dir: None,
            handlebars_dir: Some(get_lastlayer_root_dir().join("src/handlebars")),
            bin: Some(get_lastlayer_root_dir().join("verilator/build/bin/verilator")),
            reg: Vec::new(),
            mem: Vec::new(),
        }
    }

    pub fn add_register(&mut self, hid: u32, path: &str, width: u32) -> &mut Build {
        self.reg.push(Register {
            hid: hid,
            path: path.to_string(),
            width: width,
        });
        self
    }

    pub fn add_memory(&mut self, hid: u32, path: &str, width: u32) -> &mut Build {
        self.mem.push(Memory {
            hid: hid,
            path: path.to_string(),
            width: width,
        });
        self
    }

    pub fn verilog_disable_warning(&mut self, name: &str) -> &mut Build {
        self.verilog_warnings.push(name.to_string());
        self
    }

    pub fn top_module(&mut self, name: &str) -> &mut Build {
        self.top_name = Some(name.to_string());
        self.virtual_top_name = Some(format!("lastlayer_{}", self.get_top_name()));
        self
    }

    pub fn clock(&mut self, name: &str) -> &mut Build {
        self.clock_name = Some(name.to_string());
        self
    }

    pub fn reset(&mut self, name: &str) -> &mut Build {
        self.reset_name = Some(name.to_string());
        self
    }

    pub fn out_dir<P: AsRef<Path>>(&mut self, out: P) -> &mut Build {
        self.out_dir = Some(out.as_ref().to_path_buf());
        self
    }

    pub fn cc_flag(&mut self, name: &str) -> &mut Build {
        self.cc_flags.push(name.to_string());
        self
    }

    pub fn cc_include_dir<P: AsRef<Path>>(&mut self, dir: P) -> &mut Build {
        assert!(
            dir.as_ref().is_dir(),
            "include dir does not seems to be a directory"
        );
        self.cc_include_dirs.push(dir.as_ref().to_path_buf());
        self
    }

    pub fn cc_link_dir<P: AsRef<Path>>(&mut self, dir: P) -> &mut Build {
        assert!(
            dir.as_ref().is_dir(),
            "linking dir does not seems to be a directory"
        );
        self.cc_link_dirs.push(dir.as_ref().to_path_buf());
        self
    }

    pub fn cc_link_lib(&mut self, name: &str) -> &mut Build {
        self.cc_link_libs.push(name.to_string());
        self
    }

    pub fn cc_file<P: AsRef<Path>>(&mut self, file: P) -> &mut Build {
        self.cc_files.push(file.as_ref().to_path_buf());
        self
    }

    pub fn verilog_file<P: AsRef<Path>>(&mut self, file: P) -> &mut Build {
        self.verilog_files.push(file.as_ref().to_path_buf());
        self
    }

    pub fn verilog_include_dir<P: AsRef<Path>>(&mut self, dir: P) -> &mut Build {
        assert!(
            dir.as_ref().is_dir(),
            "include dir does not seems to be a directory"
        );
        self.verilog_include_dirs.push(dir.as_ref().to_path_buf());
        self
    }

    pub fn compile(&mut self, name: &str) {
        self.create_out_dir();
        self.create_virtual_verilog_top();
        self.create_virtual_cc_top();
        self.compile_awig();
        self.default_verilog_warning();
        self.compile_verilog();
        self.default_cc_files();
        self.default_include_dirs();
        self.compile_cxx(name);
        self.copy_header();
        self.create_link_to_verilator_include();
    }
}
