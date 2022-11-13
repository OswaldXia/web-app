// It's reasonable to assume that all to-do views require an
use crate::database::establish_connection;
use crate::json_serialization::to_do_items::ToDoItems;
use crate::models::item::item::Item;
use crate::schema::to_do;
use crate::to_do::to_do_factory;
use diesel::prelude::*;

pub fn return_state() -> ToDoItems {
    // establish the connection
    let connection = &mut establish_connection();
    //  get our table and build a database query from it
    let items = to_do::table
        // The first part of the query defines the order
        .order(to_do::columns::id.asc())
        // then define what struct is going to be used to load the data and pass in a reference to the connection
        .load::<Item>(connection)
        .unwrap();

    let mut items_processed = vec![];
    for item in items {
        // With the data from the database, we loop through constructing our item structs and appending them to our buffer.
        let item = to_do_factory(&item.status, &item.title).unwrap();
        items_processed.push(item);
    }
    // construct the JSON schema's ToDoItems
    ToDoItems::new(items_processed)
}
