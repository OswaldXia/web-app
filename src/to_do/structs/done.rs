use super::base::Base;
use super::traits::{delete::Delete, edit::Edit, get::Get};

pub struct Done {
    pub super_struct: Base,
}

impl Done {
    pub fn new(input_title: &str) -> Done {
        let base: Base = Base::new(input_title, "done");
        return Done { super_struct: base };
    }
}

impl Get for Done {}
impl Delete for Done {}
impl Edit for Done {}

#[cfg(test)]
mod done_tests {
    use super::*;

    #[test]
    fn new() {
        let title = &"excel date".to_string();
        let expected_title = &"excel date".to_string();
        let expected_status = &"done".to_string();

        let done = Done::new(title);
        assert_eq!(expected_title, &done.super_struct.title);
        assert_eq!(expected_status, &done.super_struct.status);
    }
}
