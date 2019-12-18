use crate::util::{get_manifest_dir, run_cmd};
use std::path::{Path, PathBuf};
use handlebars::Handlebars;
use serde::Serialize;
use std::error::Error;
use std::fs::File;
use std::process::Command;

pub struct Build {
    virtual_top: Option<String>,
    top: Option<String>,
    clock: Option<String>,
    reset: Option<String>,
    dpi: bool,
    warnings: Vec<String>,
    verilog_files: Vec<PathBuf>,
    out_dir: Option<PathBuf>,
    handlebars_dir: Option<PathBuf>,
    bin: Option<PathBuf>,
}

#[derive(Serialize)]
struct VirtualHandle {
    vtop: String,
    top: String,
    clock: String,
    reset: String,
}

impl Build {

    fn get_top(&self) -> String {
        match self.top.clone() {
            Some(p) => p,
            None => panic!("Top module name not set"),
        }
    }

    fn get_virtual_top(&self) -> String {
        match self.virtual_top.clone() {
            Some(p) => p,
            None => panic!("Virtual top module name not set"),
        }
    }

    fn get_clock(&self) -> String {
        match self.clock.clone() {
            Some(p) => p,
            None => panic!("Clock name not set"),
        }
    }

    fn get_reset(&self) -> String {
        match self.reset.clone() {
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

    fn render(
        &self,
        input: &str,
        output: &str
    ) -> Result<(), Box<dyn Error>> {
        let reg = Handlebars::new();
        let handle = VirtualHandle {
            vtop: self.get_virtual_top(),
            top: self.get_top(),
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

    pub fn new() -> Build {
        Build {
            virtual_top: None,
            top: None,
            clock: Some("clock".to_string()),
            reset: Some("reset".to_string()),
            dpi: false,
            warnings: Vec::new(),
            verilog_files: Vec::new(),
            out_dir: None,
            handlebars_dir: Some(get_manifest_dir().join("src/handlebars")),
            bin: Some(get_manifest_dir().join("verilator/build/bin/verilator")),
        }
    }

    pub fn disable_warning(&mut self, name: &str) -> &mut Build {
        self.warnings.push(name.to_string());
        self
    }

    pub fn top_module(&mut self, name: &str) -> &mut Build {
        self.top = Some(name.to_string());
        self.virtual_top = Some(format!("__{}", self.get_top()));
        self
    }

    pub fn clock(&mut self, name: &str) -> &mut Build {
        self.clock = Some(name.to_string());
        self
    }

    pub fn reset(&mut self, name: &str) -> &mut Build {
        self.reset = Some(name.to_string());
        self
    }

    pub fn dpi_flag(&mut self, flag: bool) -> &mut Build {
        self.dpi = flag;
        self
    }

    pub fn out_dir<P: AsRef<Path>>(&mut self, out: P) -> &mut Build {
        assert!(out.as_ref().is_dir(), "out_dir does not seems to be a directory");
        self.out_dir = Some(out.as_ref().to_path_buf());
        self
    }

    pub fn verilog_file<P: AsRef<Path>>(&mut self, file: P) -> &mut Build {
        self.verilog_files.push(file.as_ref().to_path_buf());
        self
    }

    fn create_out_dir(&self) {
        let mut cmd = Command::new("mkdir");
        cmd.arg("-p")
            .arg(self.get_out_dir());
        run_cmd(&mut cmd);
    }

    fn create_virtual_top(&mut self) -> &mut Build {
        let virtual_name = "virtual_top.v";
        let virtual_hbs = format!("{}.hbs", &virtual_name);
        let virtual_file = self.get_out_dir().join(&virtual_name);
        self.create_out_dir();
        self.render(&virtual_hbs, &virtual_name).expect("failed to render virtual top");
        self.verilog_file(&virtual_file);
        self
    }

    fn run_verilator(&self) {
        let mut cmd = Command::new(self.get_bin());
        cmd.arg("--cc")
            .arg("-Mdir")
            .arg(self.get_out_dir())
            .arg("--top-module")
            .arg(self.get_virtual_top());
        for file in self.verilog_files.iter() {
            cmd.arg(file);
        }
        for warn in &self.warnings {
            cmd.arg(format!("-Wno-{}", warn));
        }
        run_cmd(&mut cmd);
    }

    pub fn compile_verilog(&mut self) -> &mut Build {
        self.create_virtual_top();
        self.run_verilator();
        self
    }
}

// pub struct Build {
//     top: Option<String>,
//     dpi: bool,
//     out: Option<PathBuf>,
//     verilog_files: Vec<PathBuf>,
//     cc_files: Vec<PathBuf>,
//     sim_filename: String,
//     sim_dpi_filename: String,
//     lib_name: Option<String>,
//     inclue_paths: Vec<PathBuf>,
//     warnings: Vec<String>,
// }

// impl Build {

//     fn render_template(
//         &self,
//         def: &HandleBars,
//         template_path: &Path,
//         output_path: &Path,
//     ) -> Result<(), Box<dyn Error>> {
//         let reg = Handlebars::new();
//         let mut template = File::open(template_path)?;
//         let mut output_file = File::create(output_path)?;
//         reg.render_template_source_to_write(&mut template, def, &mut output_file)?;
//         Ok(())
//     }

//     fn get_include_path(&self) -> PathBuf {
//         let mut p: Option<PathBuf> = None;
//         for path in self.inclue_paths.iter() {
//             if path.is_dir() {
//                 p = Some(path.to_path_buf());
//                 break;
//             }
//         }
//         match p {
//             Some(t) => t,
//             None => panic!("Verilator include library not found"),
//         }
//     }


//     fn compile_cc(&mut self) {
//         let include_path = self.get_include_path();
//         let out_path = self.get_out_path();
//         let mut cmd = Command::new("g++");

//         cmd.arg("-shared")
//             .arg("-faligned-new")
//             .arg("-shared")
//             .arg("-fPIC")
//             .arg("-I")
//             .arg(&out_path)
//             .arg("-I")
//             .arg(&include_path)
//             .arg("-I")
//             .arg(&include_path.join("vltstd"))
//             .arg(&include_path.join("verilated.cpp"))
//             .arg(&out_path.join(format!("V{}.cpp", self.get_top())))
//             .arg(&out_path.join(format!("V{}__Syms.cpp", self.get_top())));

//         for file in self.cc_files.iter() {
//             cmd.arg(file);
//         }

//         if self.dpi {
//             cmd.arg(&include_path.join("verilated_dpi.cpp"));
//             cmd.arg(&out_path.join(format!("V{}__Dpi.cpp", self.get_top())));
//         }

//         cmd.arg("-o").arg(self.get_shared_lib_path());

//         run_cmd(&mut cmd);
//     }

//     fn set_shared_lib_name(&mut self, name: &str) {
//         self.lib_name = Some(name.to_string());
//     }

//     fn get_shared_lib_name(&self) -> String {
//         match self.lib_name.clone() {
//             Some(s) => s.to_string(),
//             None => panic!("Shared library name not set"),
//         }
//     }

//     pub fn get_shared_lib_path(&self) -> PathBuf {
//         let path = self.get_out_path();
//         path.join(format!("lib{}.so", self.get_shared_lib_name()))
//     }

//     pub fn new() -> Build {
//         Build {
//             top: None,
//             dpi: false,
//             out: None,
//             verilog_files: Vec::new(),
//             cc_files: Vec::new(),
//             sim_filename: String::from("verilator_sim"),
//             sim_dpi_filename: String::from("verilator_sim_dpi"),
//             lib_name: None,
//             inclue_paths: vec![
//                 PathBuf::from("/usr/share/verilator/include"),
//                 PathBuf::from("/usr/local/share/verilator/include"),
//             ],
//             warnings: Vec::new(),
//         }
//     }

//     pub fn warn_width(&mut self, flag: bool) -> &mut Build {
//         if !flag {
//             self.warnings.push("WIDTH".to_string());
//         }
//         self
//     }



//     pub fn dpi_flag(&mut self, flag: bool) -> &mut Build {
//         self.dpi = flag;
//         self
//     }

//     pub fn out_dir<P: AsRef<Path>>(&mut self, out: P) -> &mut Build {
//         self.out = Some(out.as_ref().to_path_buf());
//         self
//     }

//     pub fn verilog_file<P: AsRef<Path>>(&mut self, file: P) -> &mut Build {
//         self.verilog_files.push(file.as_ref().to_path_buf());
//         self
//     }

//     pub fn cc_file<P: AsRef<Path>>(&mut self, file: P) -> &mut Build {
//         self.cc_files.push(file.as_ref().to_path_buf());
//         self
//     }

//     pub fn compile(&mut self, name: &str) {
//         let top = self.get_top();
//         self.compile_verilog();
//         self.render_template(
//             &HandleBars {
//                 name: top.to_string(),
//             },
//             &self.get_hbs_path(&self.sim_filename),
//             &self.get_cc_path(&self.sim_filename),
//         )
//         .expect("failed to handlebar sim file");
//         self.cc_file(self.get_cc_path(&self.sim_filename));
//         if self.dpi {
//             self.render_template(
//                 &HandleBars {
//                     name: top.to_string(),
//                 },
//                 &self.get_hbs_path(&self.sim_dpi_filename),
//                 &self.get_cc_path(&self.sim_dpi_filename),
//             )
//             .expect("failed to handlebar sim dpi file");
//             self.cc_file(self.get_cc_path(&self.sim_dpi_filename));
//         }
//         self.set_shared_lib_name(name);
//         self.compile_cc();
//     }
// }