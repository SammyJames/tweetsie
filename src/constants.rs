
use secstr::*;
use std::string::String;

lazy_static! {
	pub static ref CONSUMER_SEC: SecStr = SecStr::from(include_str!("../res/secret"));
	pub static ref CONSUMER_KEY: SecStr = SecStr::from(include_str!("../res/key"));
	pub static ref APP_ONLY_AUTH: String = String::from("https://api.twitter.com/oauth2/token");
	pub static ref REQ_TOKEN_URL: String = String::from("https://api.twitter.com/oauth/request_token");
	pub static ref AUTHORIZE_URL: String = String::from("https://api.twitter.com/oauth/authorize");
	pub static ref ACC_TOKEN_URL: String = String::from("https://api.twitter.com/oauth/access_token");
}
