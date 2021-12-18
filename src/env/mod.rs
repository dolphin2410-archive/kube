use std::process::{Command, Stdio};

#[cfg(target_os = "windows")]
const PATH: &str = ";";

#[cfg(target_family = "unix")]
const PATH: &str = ":";

pub fn add_path(path: &str) {
    set_value("Path", &format!("{}{}{}", path, PATH, std::env::var("PATH").unwrap()));
}

#[cfg(target_family = "windows")]
pub fn set_value(key: &str, value: &str) {
    println!("{}", value);
    let mut child = Command::new("setx")
        .args(vec!["/m", key, value])
        .stdout(Stdio::inherit())
        .spawn()
        .unwrap();

    child.wait()
        .unwrap();
}

#[cfg(target_family = "unix")]
pub fn set_value(key: &str, value: &str) {
    let str = format!("\"export {}={}\"", key, value);
    let mut child = Command::new("sudo")
        .args(vec!["echo", &str.as_str(), ">>", "/etc/environment"])
        .stdout(Stdio::inherit())
        .spawn()
        .unwrap();

    child.wait()
        .unwrap();

}