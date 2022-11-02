/*
To demonstrate this, we are going to build a basic view that takes a parameter from the
URL and creates a to do item. To do this, we will have to do the following:
1. Load the current state of the to do item list.
2. Get the title of the new to do item from the URL.
3. Pass the title and the pending string through to_do_factory.
4. Pass the result of the previous step, along with the create string and the state, into
the process module interface.
5. Return a string to the user to signal that the process has finished.
*/

use std::fmt::format;

use crate::processes::process_input;
use crate::state::read_file;
use crate::to_do::{self, to_do_factory};
use actix_web::HttpRequest;
use serde_json::value::Value;
use serde_json::Map;

pub async fn create(req: HttpRequest) -> String {
    let file_name = "./state.json";
    let state = read_file(file_name); // 1

    let title = req // 2
        .match_info()
        .get("title")
        .unwrap()
        .to_string();

    let item = to_do_factory("pending", &title).expect("create"); // 3

    process_input(item, "create".to_string(), &state); // 4
    return format!("{} created", title); // 5
}
