use serde::Deserialize;

// Extracting data from the request body is fairly straightforward. All we have to do is define a struct with the attributes we want, and then pass that through as a parameter in the view function.
#[derive(Deserialize)]
pub struct ToDoItem {
    pub title: String,
    pub status: String,
}
