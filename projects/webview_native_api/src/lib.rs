pub mod command_types;
mod native_apis;
mod native_handler;

use native_handler::*;
use webview_official;

pub fn provide_native_api(mut webview: webview_official::Webview) {
    let w = webview.clone();
    webview.bind("__native__", create_native_api(&w));
}

pub use native_handler::create_handler;
pub use native_handler::CommandResult;
