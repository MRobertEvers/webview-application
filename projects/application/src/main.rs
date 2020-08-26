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

pub fn provide_api<'a>(mut webview: webview_official::Webview) {
    let mut w = webview.clone();
    webview.bind("__application__", create_handler(&w, handle_command));
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
    provide_native_api(w);

    let mut w2 = webview.clone();
    provide_api(w2);
    webview.run();
}
