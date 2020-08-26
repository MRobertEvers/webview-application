pub mod command_types;
mod native_apis;
mod native_handler;

use native_handler::*;
use webview_official;

pub fn provide_native_api<'a>(webview: &mut webview_official::Webview) {
    let mut w = webview.clone();
    w.bind("__native__", create_native_api(webview));
}

pub use native_handler::create_handler;
pub use native_handler::CommandResult;
