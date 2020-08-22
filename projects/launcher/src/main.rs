mod command_types;
mod native_apis;
mod native_handler;

use native_handler::*;
use webview_official;

fn create_handler<'a>(webview: &'a webview_official::Webview) -> impl FnMut(&str, &str) + 'a {
    move |sequence: &str, string_args: &str| {
        println!("{}", string_args);
        webview.r#return(sequence, 0, string_args);
    }
}

/**
 * Called immediately when the window is ready.
 */
fn dispatch_callback(w: &mut webview_official::Webview) {
    // w.set_size(800, 600, webview_official::SizeHint::MIN);
}

fn main() {
    println!("Wow");

    let builder = webview_official::WebviewBuilder::new();

    let mut webview = builder
        .debug(true)
        .width(1024)
        .height(768)
        .resize(webview_official::SizeHint::NONE)
        .dispatch(dispatch_callback)
        .url("http://localhost:4040")
        // .url(
        //     r#"data:text/html,
        //     <html>
        //     <body>
        //         <p>A big black bear</p>
        //         <button id="butt">Click me!</button>
        //     </body>
        //     <script>
        //         let butt = document.getElementById("butt");
        //         butt.onclick = function() {
        //             window.native("This", "is", "a", 1).then(function(res) {
        //                 console.log(res);
        //             }, function(err) {
        //                 console.log(err);
        //             });
        //         }
        //     </script>
        //     </html>
        // "#,
        // )
        .build();
    let w = webview.clone();
    webview.bind("echo", create_handler(&w));
    webview.bind("__native__", create_native_api(&w));
    webview.run();
}
