// The purpose of this module is to direct the flow of the commands.
// We need an entry point to process the input and direct it to the right function to process the item.

use super::to_do::structs::done::Done;
use super::to_do::structs::pending::Pending;
use super::to_do::structs::traits::create::Create;
use super::to_do::structs::traits::delete::Delete;
use super::to_do::structs::traits::edit::Edit;
use super::to_do::structs::traits::get::Get;
use super::to_do::ItemTypes;
use serde_json::value::Value;
use serde_json::Map;

//  define the functions that enable us to process Pending structs
fn process_pending(item: Pending, command: &str, state: &Map<String, Value>) {
    let mut state = state.clone();
    match command {
        "get" => item.get(&item.super_struct.title, &state),
        "create" => item.create(
            &item.super_struct.title,
            &item.super_struct.status,
            &mut state,
        ),
        "delete" => item.delete(&item.super_struct.title, &mut state),
        "edit" => item.set_to_done(&item.super_struct.title, &mut state),
        _ => println!("command: {} not supported", command),
    }
}

// define the functions that enable us to process Done structs
fn process_done(item: Done, command: &str, state: &Map<String, Value>) {
    let mut state = state.clone();
    match command {
        "get" => item.get(&item.super_struct.title, &state),
        "delete" => item.delete(&item.super_struct.title, &mut state),
        "edit" => item.set_to_pending(&item.super_struct.title, &mut state),
        _ => println!("command: {} not supported", command),
    }
}

// build an entry point that takes a struct, memory state, and command so we can funnel the struct into the right function:
pub fn process_input(item: ItemTypes, command: &str, state: &Map<String, Value>) {
    match item {
        ItemTypes::Pending(item) => process_pending(item, command, state),
        ItemTypes::Done(item) => process_done(item, command, state),
    }
}
