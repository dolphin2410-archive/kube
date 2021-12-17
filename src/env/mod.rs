use std::process::{Command, Stdio};

#[cfg(target_os = "windows")]
const PATH: &str = ";%PATH%";

#[cfg(target_family = "unix")]
const PATH: &str = ":$PATH";

pub fn add_path(path: &str) {
    let path = path.to_owned() + PATH;
    set_value("PATH", &path.as_str());
}

#[cfg(target_family = "windows")]
pub fn set_value(key: &str, value: &str) {
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