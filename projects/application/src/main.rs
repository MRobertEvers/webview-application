use webview_native_api::{command_types, create_handler, provide_native_api, CommandResult};
use webview_official;

fn handle_command(command: command_types::Command, resolve: &mut dyn FnMut(CommandResult, &str)) {
    println!("wow");
}

fn main() {
    let builder = webview_official::WebviewBuilder::new();

    let mut webview = builder
        .debug(true)
        .width(1024)
        .height(768)
        .resize(webview_official::SizeHint::NONE)
        .url("http://localhost:4040")
        .build();

    let w = webview.clone();
    provide_native_api(w);

    let w2 = webview.clone();
    let _handler = create_handler(&w2, handle_command);
    webview.run();
}
