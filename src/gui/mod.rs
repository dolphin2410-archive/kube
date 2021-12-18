use nfd::Response;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufWriter;
use std::path::Path;
use web_view::Content;

// 발코딩 ㅅㄱ

const ZIP_ARCHIVE: &[u8] = include_bytes!(concat!("../../", include_str!("../../target.txt")));

const APP_NAME: &str = include_str!("../../app.txt");

pub fn root() -> &'static str {
    include_str!("../../resources/root.html")
}

pub fn next_page() -> &'static str {
    include_str!("../../resources/next_page.html")
}

pub fn render(html: &str) {
    web_view::builder()
        .title("kube")
        .content(Content::Html(html))
        .size(800, 500)
        .resizable(false)
        .user_data(())
        .invoke_handler(|webview, arg| {
            match arg {
                "hello" => {
                    println!("Hello, World!");
                }
                "open_file" => {
                    let result = nfd::open_pick_folder(None).unwrap();
                    match result {
                        Response::Okay(path) => {
                            webview
                                .eval(&format!(r#"update_folder("{}")"#, path.replace("\\", "/")))
                                .unwrap();
                        }
                        _ => {}
                    }
                }
                "exit" => {
                    webview.exit();
                }
                _ => {
                    if arg.starts_with("next_page:") {
                        let pathname = arg.replace("next_page:", "");

                        let pathname_str = pathname.as_str();

                        let path = Path::new(pathname_str);

                        let mut success = true;

                        if let Some(parent) = path.parent() {
                            if !parent.exists() {
                                webview.eval("no_file()").unwrap();
                                success = false;
                            }
                        }

                        if success {
                            let exists = path.exists();

                            if !exists || (exists && path.read_dir().unwrap().next().is_none()) {
                                if !exists {
                                    std::fs::create_dir(path).unwrap();
                                }

                                webview
                                    .eval(&format!(
                                        "move_page('{}')",
                                        next_page().replace("\r", "").replace("\n", "\\n")
                                    ))
                                    .unwrap();
                                let mut name = String::from(pathname_str);
                                name.push_str("/");
                                name.push_str(APP_NAME);
                                name.push_str(".zip");

                                let mut file =
                                    BufWriter::new(File::create(&name.as_str()).unwrap());
                                file.write_all(ZIP_ARCHIVE).unwrap();
                                webview.eval(&format!("update_size(20)")).unwrap();
                                let mut name2 = String::from(pathname_str);
                                name2.push_str("/");
                                name2.push_str(APP_NAME);
                                let mut archive =
                                    zip::ZipArchive::new(File::open(&name.as_str()).unwrap())
                                        .unwrap();
                                std::fs::create_dir(&name2.as_str()).unwrap();
                                archive.extract(Path::new(&name2.as_str())).unwrap();
                                webview.eval(&format!("update_size(80)")).unwrap();
                                name2.push_str("/bin");
                                crate::env::add_path(&name2.as_str());
                                std::fs::remove_file(Path::new(&name.as_str())).unwrap();
                                webview.eval(&format!("update_size(100)")).unwrap();
                                webview.eval(&format!("enable_btn()")).unwrap();
                            } else {
                                webview.eval("yes_file()").unwrap();
                            }
                        }
                    }
                }
            };

            Ok(())
        })
        .build()
        .unwrap()
        .run()
        .unwrap();
}

pub fn start() {
    render(root());
}
