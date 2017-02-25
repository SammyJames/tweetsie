pub mod local_user;
pub mod user;
pub mod tweet;
pub mod tweet_stream;

use model::local_user::LocalUser;

#[derive(Debug)]
pub struct TweetsieModel {
    local_user: LocalUser,
}

impl TweetsieModel {
    pub fn new() -> TweetsieModel {

        TweetsieModel { local_user: LocalUser::new() }
    }

    pub fn is_logged_in<'a>(&'a self) -> bool {
        false
    }
}
