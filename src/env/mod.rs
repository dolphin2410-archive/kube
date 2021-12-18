use std::process::{Command, Stdio};

#[cfg(target_os = "windows")]
/// The Windows Path Separator
const PATH_SEPARATOR: &str = ";";


#[cfg(target_family = "unix")]
// THe Unix Path Separator
const PATH_SEPARATOR: &str = ":";

/// Add the given Path to the system's 'PATH' environment variable
pub fn add_path(path: &String) {
    set_value("PATH", &format!("{}{}{}", path, PATH_SEPARATOR, std::env::var("PATH").unwrap()));
}

#[cfg(target_family = "windows")]
/// Set Environment Variable value for windows
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
/// Set Environment Variable value for Unix Based Systems
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