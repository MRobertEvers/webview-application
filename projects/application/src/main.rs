use webview_native_api::{create_handler, provide_native_api, CommandResult};
use webview_official;

#[derive(serde::Deserialize)]
#[serde(tag = "command", rename_all = "camelCase")]
pub enum Command {
    /// The read text file API.
    Read { message: String },
}

fn handle_command(command: Command, resolve: &mut dyn FnMut(CommandResult, &str)) {
    println!("wow");
    resolve(CommandResult::SUCCESS, "good job");
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

    let mut w = webview.clone();
    provide_native_api(&mut w);

    let mut w2 = webview.clone();
    w2.bind("__application__", create_handler(&webview, handle_command));
    w2.run();
}
