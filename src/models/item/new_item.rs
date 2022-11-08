// define a to-do item data model struct for insertion
use crate::schema::to_do;
use diesel::Insertable;

// stating that we allow the data to be inserted into the database with the Insertable tag.
#[derive(Insertable)]
#[table_name = "to_do"]
pub struct NewItem {
    pub title: String,
    pub status: String,
}

impl NewItem {
    pub fn new(title: &str) -> NewItem {
        NewItem {
            title: title.to_string(),
            status: "pending".to_string(),
        }
    }
}
