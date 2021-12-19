use std::path::Path;
use std::process::Command;
use tokio::fs;

pub async fn unzip(source: &str, target: &str) {
    fs::create_dir_all(Path::new(target)).await.unwrap();
    unzip_cmd(source, target);
}

#[cfg(target_family = "windows")]
pub fn unzip_cmd(source: &str, target: &str) {
    let result = Command::new("tar")
        .args(vec!["-xf", source , "-C", target])
        .output()
        .unwrap();

    String::from_utf8(result.stdout);
}

#[cfg(target_family = "unix")]
pub fn unzip_cmd(source: &str, target: &str) {
    Command::new("unzip")
        .args(vec![source, "-d", target])
        .output()
        .unwrap();
}