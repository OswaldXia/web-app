/// This struct is going to be used to define a standard prefix for a URL.
pub struct Path {
    pub prefix: String,
    // declare whether the struct instance is for a backend endpoint
    pub backend: bool
}

impl Path {
    pub fn define(&self, following_path: &str) -> String {
        if self.backend {
            format!("/api/v1{}", self.prefix.clone() + following_path)
        } else {
            self.prefix.clone() + following_path
        }
    }
}