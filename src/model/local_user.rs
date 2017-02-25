

use std::string::String;

use model::user::TwitterUser;
use model::user::User;

#[derive(Debug)]
pub struct LocalUser {
    consumer_key: String,
    consumer_secret: String,
    token: String,
    token_secret: String,
    user: User,
}

impl LocalUser {
    pub fn new() -> LocalUser {

        LocalUser {
            consumer_key: String::from(""),
            consumer_secret: String::from(""),
            token: String::from(""),
            token_secret: String::from(""),
            user: User::new(),
        }
    }

    pub fn get_consumer_key(&self) -> &String {
        &self.consumer_key
    }

    pub fn get_consumer_secret(&self) -> &String {
        &self.consumer_secret
    }

    pub fn get_token(&self) -> &String {
        &self.token
    }

    pub fn get_token_secret(&self) -> &String {
        &self.token_secret
    }
}

impl TwitterUser for LocalUser {
    fn get_display_name(&self) -> &String {
        self.user.get_display_name()
    }

    fn get_handle(&self) -> &String {
        self.user.get_handle()
    }
}
