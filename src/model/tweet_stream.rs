
use twitter::{StreamMessage, TwitterStream};
use model::local_user::LocalUser;

pub struct TweetStream {}

impl TweetStream {
    pub fn new(user: LocalUser) -> TweetStream {

        let stream = TwitterStream::user(user.get_consumer_key(),
                                         user.get_consumer_secret(),
                                         user.get_token(),
                                         user.get_token_secret())
            .unwrap();

        TweetStream {}
    }
}
