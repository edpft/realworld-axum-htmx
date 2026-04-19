use super::value_objects::{Email, Id, Name};

pub struct User {
    id: Id,
    email: Email,
    name: Name,
}
