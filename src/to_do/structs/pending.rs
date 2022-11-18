use super::base::Base;
use super::traits::{create::Create, delete::Delete, edit::Edit, get::Get};
pub struct Pending {
    pub super_struct: Base,
}
impl Pending {
    pub fn new(input_title: &str) -> Pending {
        let base: Base = Base::new(input_title, "pending");
        return Pending { super_struct: base };
    }
}

impl Create for Pending {}
impl Delete for Pending {}
impl Edit for Pending {}
impl Get for Pending {}

#[cfg(test)]
mod pending_tests {
    use super::*;

    #[test]
    fn new() {
        let title = &"washing".to_string();
        let expected_title = &"washing".to_string();
        let expected_status = &"pending".to_string();

        let pending = Pending::new(title);
        assert_eq!(expected_title, &pending.super_struct.title);
        assert_eq!(expected_status, &pending.super_struct.status);
    }
}