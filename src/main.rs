extern crate twitter_stream as twitter;
extern crate gtk;

mod main_window;
use main_window::MainWindow;

mod tweet_stream;
use tweet_stream::TweetStream;

mod tweet;
use tweet::Tweet;

mod user;
use user::User;

mod local_user;
use local_user::LocalUser;

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
