
use model::user::User;

#[derive(Debug)]
pub struct Tweet {
    user: User,
}

impl Tweet {
	pub fn new(user: User) -> Tweet {
		Tweet {
			user: user,
		}
	}
}