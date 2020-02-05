use std::env::set_current_dir;
use std::path::{Path, PathBuf};
use std::process::Command;

pub fn get_lastlayer_root_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

pub fn create_date_dir(path: &Path, name: &str) -> PathBuf {
    use chrono::{DateTime, Local};
    let now: DateTime<Local> = Local::now();
    let mut p = PathBuf::new();
    p.push(path);
    p.push(name);
    p.push(format!("{}", now.format("%Y_%m_%d_%H_%M_%S")));
    p
}

pub fn create_dir(path: &Path) -> std::io::Result<()> {
    use std::fs::create_dir_all;
    create_dir_all(path)?;
    Ok(())
}

pub fn run_cmd(cmd: &mut Command) {
    let status = match cmd.status() {
        Ok(status) => status,
        Err(_) => panic!("failed to execute command"),
    };
    if !status.success() {
        panic!("command did not execute successfully");
    }
}

pub fn change_dir(dir: &Path) {
    set_current_dir(dir).expect("failed to change dir");
}
