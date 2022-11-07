// define a to-do item data model struct for management
use crate::schema::to_do;
use diesel::{Identifiable, Queryable};

#[derive(Queryable, Identifiable)]
#[table_name = "to_do"]
pub struct Item {
    pub id: i32,
    pub title: String,
    pub status: String,
}
