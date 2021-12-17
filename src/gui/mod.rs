use std::fs::File;
use std::io::Read;
use nfd::Response;
use web_view::Content;

pub fn start() {
    let mut file = File::open("hello.html").unwrap();
    let mut str = String::new();
    file.read_to_string(&mut str).unwrap();
    let html_content = &str.as_str();

    web_view::builder()
        .title("Kube")
        .content(Content::Html(html_content))
        .size(800, 500)
        .resizable(false)
        .user_data(())
        .invoke_handler(|webview, arg| {
            match arg {
                "hello" => {
                    println!("Hello, World!")
                },
                "open_file" => {
                    let result = nfd::open_pick_folder(None).unwrap();
                    match result {
                        Response::Okay(path) => {
                            webview.eval(&format!("update_folder(\"{}\")", path.replace("\\", "/"))).unwrap();
                        },
                        _ => {}
                    }
                },
                _ => unimplemented!(),
            };

            Ok(())
        })
        .run()
        .unwrap();
}