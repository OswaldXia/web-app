// reading an HTML file to a string, and then passing it to the body to be returned. Finnaly the browser can render the HTML string in the response body

use std::fs;

pub fn read_file(file_path: &str) -> String {
    // All we have to do here is return a string because this is all we need for the response body.
    fs::read_to_string(file_path).expect("unable to read file")
}
