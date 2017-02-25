
use std::string::String;

#[derive(Debug)]
pub struct User {
    display_name: String,
    handle: String,
}


impl User {
    pub fn new() -> User {

        User {
            display_name: String::from("Display Name"),
            handle: String::from("@handle"),
        }
    }
}

pub trait TwitterUser {
    fn get_display_name(&self) -> &String;
    fn get_handle(&self) -> &String;
}

impl TwitterUser for User {
    fn get_display_name(&self) -> &String {
        &self.display_name
    }

    fn get_handle(&self) -> &String {
        &self.handle
    }
}
