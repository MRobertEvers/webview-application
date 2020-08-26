use crate::command_types;
use crate::native_apis::*;
use serde_json::to_string;
use webview_official::Webview;

fn handle_command(command: command_types::Command, resolve: &mut dyn FnMut(CommandResult, &str)) {
    use command_types::Command::*;
    match command {
        Print { message } => {
            println!("{}", message);
            resolve(CommandResult::SUCCESS, &message);
        }
        FileDialog => {
            let dialog_result = file_dialog::file_dialog();
            match dialog_result {
                Ok(filepath) => {
                    // Make the result 'eval' friendly.
                    let formatted_result = serde_json::to_string(&filepath).unwrap();
                    resolve(CommandResult::SUCCESS, &formatted_result);
                }
                Err(error_message) => {
                    // let mut formatted_result = format!("'{}'", error_message);
                    // formatted_result = formatted_result.replace("\\", "\\\\");
                    let formatted_result = serde_json::to_string(&error_message).unwrap();
                    resolve(CommandResult::FAILURE, &formatted_result);
                }
            }
        }
        ReadFile { filename } => {
            let result = std::fs::read_to_string(filename).unwrap();
            let formatted_result = serde_json::to_string(&result).unwrap();
            resolve(CommandResult::SUCCESS, &formatted_result);
        }
    }
}

// Strip array characters, [...] => ...
fn strip_array_chars(string_array: &str) -> String {
    string_array
        .chars()
        .skip(1)
        .take(string_array.len() - 2)
        .collect::<String>()
}

pub enum CommandResult {
    SUCCESS,
    FAILURE,
}

fn create_resolver<'a>(
    webview: &'a Webview,
    sequence: &'a str,
) -> impl FnMut(CommandResult, &str) + 'a {
    move |res: CommandResult, message: &str| {
        let converted_result = match res {
            CommandResult::SUCCESS => 0,
            CommandResult::FAILURE => 1,
        };
        webview.r#return(sequence, converted_result, message);
    }
}

/**
 * Creates a callback function (closure) that accepts javascript calls that look like
 *
 * window[<name>]({...})
 *
 * This results in a string_args_array that looks like [{...}]
 */
pub fn create_native_api<'a>(webview: &'a Webview) -> impl FnMut(&str, &str) + 'a {
    move |sequence: &str, string_args_array: &str| {
        println!("{}", string_args_array);
        let string_args_object = strip_array_chars(string_args_array);
        let parse_result = serde_json::from_str(&string_args_object);

        let mut resolver = create_resolver(webview, sequence);

        match parse_result {
            Ok(command) => {
                handle_command(command, &mut resolver);
            }
            _ => (),
        };
    }
}

/**
 * This does the same thing has 'create_native_api' but more abstracted. 'create_native_api' is left for readability.
 */
pub fn create_handler<'a>(
    webview: &'a Webview,
    handler: impl FnMut(command_types::Command, &mut dyn FnMut(CommandResult, &str)) + 'a,
) -> impl FnMut(&str, &str) + 'a {
    // Required because the closure will try to move the reference.
    let mut handler_callback = handler;
    move |sequence: &str, string_args_array: &str| {
        let string_args_object = strip_array_chars(string_args_array);
        let parse_result = serde_json::from_str(&string_args_object);

        let mut resolver = create_resolver(webview, sequence);

        match parse_result {
            Ok(command) => {
                handler_callback(command, &mut resolver);
            }
            _ => (),
        };
    }
}
