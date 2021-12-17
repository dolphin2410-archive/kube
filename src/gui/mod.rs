use nfd::Response;
use web_view::Content;

pub fn start() {
    let html_content = include_str!("../../resources/root.html");
    let _target_zip = include_bytes!(include_str!("../../target.txt"));

    web_view::builder()
        .title("kube")
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
                "next_page" => {
                    todo!("Must do!")
                }
                _ => unimplemented!()
            };

            Ok(())
        })
        .run()
        .unwrap();
}