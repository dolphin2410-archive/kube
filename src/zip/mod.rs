use std::process::Command;

#[cfg(target_family = "windows")]
pub fn unzip(source: &str, target: &str) {
    Command::new("tar")
        .args(vec!["-xf", source , "-C", target])
        .output()
        .unwrap();
}

#[cfg(target_family = "unix")]
pub fn unzip(source: &str, target: &str) {
    Command::new("unzip")
        .args(vec![source, "-d", target])
        .output()
        .unwrap();
}
