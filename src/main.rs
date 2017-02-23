extern crate twitter_stream as twitter;
extern crate gtk;

mod view;
mod control;

use view::main_window::MainWindow;
use view::tweet_stream::TweetStream;
use view::tweet::Tweet;

use control::user::User;
use control::local_user::LocalUser;

fn main() {
    if gtk::init().is_err() {
        panic!("failed to initialize gtk");
        return;
    }

    let mut local_user = LocalUser::new();

    let mut window = MainWindow::new();
    window.show();

    for _ in 0..3 {
        let mut test_stream = TweetStream::new();
        for _ in 0..10 {
            test_stream.add_tweet(Tweet::new());
        }

        window.add_stream(test_stream);
    }

    gtk::main();
}
