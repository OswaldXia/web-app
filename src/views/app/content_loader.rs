// reading an HTML file to a string, and then passing it to the body to be returned. Finnaly the browser can render the HTML string in the response body

use std::fs;

pub fn read_file(file_path: &str) -> String {
    // All we have to do here is return a string because this is all we need for the response body.
    fs::read_to_string(file_path).expect("unable to read file")
}

// Inheriting components: create a add_component function that takes the name of the component, creates tags from the component name, and loads the HTML and CSS based on the component's name
pub fn add_component(component_name: &str, html_data: &str) -> String {
    let base_path = "./templates/components";
    // load the header.html and header.css files
    let html_tag = component_name.to_uppercase() + "_HTML";
    let html_path = format!("{}/{}", base_path, component_name.to_lowercase() + ".html");
    let html_loaded = read_file(&html_path);

    let css_tag = component_name.to_uppercase() + "_CSS";
    let css_path = format!("{}/{}", base_path, component_name.to_lowercase() + ".css");
    let css_loaded = read_file(&css_path);
    // then injected the component HTML and CSS into the view data.
    html_data
        .replace(&html_tag, &html_loaded)
        .replace(&css_tag, &css_loaded)
}
