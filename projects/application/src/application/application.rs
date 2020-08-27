use super::command_types::ApplicationOperation;
use crate::database::database::read_card_by_uuid;
use webview_native_api::CommandResult;

pub fn handle_application(command: ApplicationOperation, resolve: &dyn Fn(CommandResult, &str)) {
    use crate::application::command_types::ApplicationOperation::*;
    match command {
        GetCardByUuid { uuid } => {
            let card = read_card_by_uuid(&uuid);
            // The string has to be json-stringified because it is 'eval'd.
            let formatted_result = serde_json::to_string(&card.unwrap().name).unwrap();
            resolve(CommandResult::SUCCESS, &formatted_result);
        }
        GetCardImage { uuid } => {
            resolve(CommandResult::SUCCESS, "good job");
        }
    }
}
