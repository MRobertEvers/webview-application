use webview_native_api::{create_handler, provide_native_api, CommandResult};
use webview_official;

#[derive(serde::Deserialize)]
#[serde(tag = "command", rename_all = "camelCase")]
pub enum Command {
    /// The read text file API.
    Read { message: String },
}

fn handle_command(command: Command, resolve: &dyn Fn(CommandResult, &str)) {
    println!("wow");
    resolve(CommandResult::SUCCESS, "good job");
}

pub fn provide_api<'a>(webview: &mut webview_official::Webview<'a>) {
    // Create a copy that can be moved from!
    let w = webview.clone();
    webview.bind("__application__", create_handler(w, handle_command));
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

    provide_native_api(&mut webview);

    provide_api(&mut webview);
    webview.run();
}
