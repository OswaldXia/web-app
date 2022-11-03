// It's reasonable to assume that all to-do views require an 
use crate::json_serialization::to_do_items::ToDoItems;
use crate::state::read_file;
use crate::to_do::to_do_factory;

pub fn return_state() -> ToDoItems {
    let file_name = "./state.json";
    // use read_file interface to get the state from the JSON file
    let state = read_file(file_name);

    let mut items = vec![];
    // loop through the map, converting the item type into a string and feeding it into our to_do_factory interface.
    for (key, value) in state {
        let item_type = value.as_str().unwrap();
        let item_title = key.as_str();
        let item = to_do_factory(item_type, item_title).unwrap();
        // Once we have the constructed item from the factory, we append it to a vector
        items.push(item);
    }
    // Responder has been implemented for ToDoItems, so it can be directly returned, instead of using web::Json()
    ToDoItems::new(items)
}
