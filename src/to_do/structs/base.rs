use serde::Serialize;

#[derive(Serialize)]
pub struct Base {
    pub title: String,
    pub status: String,
}

impl Base {
    pub fn new(input_title: &str, input_status: &str) -> Base {
        return Base {
            title: input_title.to_string(),
            status: input_status.to_string(),
        };
    }
}

#[cfg(test)]
mod base_tests {
    use super::*;

    // An attribute is simply metadata applied to modules and functions. This metadata aids the compiler by giving it information. In this case, it is telling the compiler that this module is a test module and that the function is an individual test. 
    #[test]
    fn new() {
        let title = &"test title".to_string();
        let expected_title = &"test title".to_string();
        let status = &"test status".to_string();
        let expected_status = &"test status".to_string();

        let new_base_struct = &Base::new(title, status);
        assert_eq!(expected_title, &new_base_struct.title);
        assert_eq!(expected_status, &new_base_struct.status);
    }
}
