use nfd::Response;
use web_view::Content;
use std::path::Path;

const _TARGET_ZIP: &[u8] = include_bytes!(concat!("../../", include_str!("../../target.txt")));


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
                },
                "open_file" => {
                    let result = nfd::open_pick_folder(None).unwrap();
                    match result {
                        Response::Okay(path) => {
                            webview.eval(&format!(r#"update_folder("{}")"#, path.replace("\\", "/"))).unwrap();
                        },
                        _ => {}
                    }
                },
                _ => {
                    if arg.starts_with("next_page:") {
                        let path = arg.replace("next_page:", "");
                        
                        let exists = Path::new(&path.as_str()).exists();

                        if !exists {
                            webview.eval("no_file()").unwrap();
                        } else {
                            println!("WOW");
                            webview.eval(&format!(r#"move_page("{}")"#, next_page())).unwrap();
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