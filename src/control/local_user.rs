

use std::string::String;

use control::user::TwitterUser;
use control::user::User;

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
}

impl TwitterUser for LocalUser {
	fn get_display_name(&self) -> &String {
		self.user.get_display_name()
	}

	fn get_handle(&self) -> &String {
		self.user.get_handle()
	}
}