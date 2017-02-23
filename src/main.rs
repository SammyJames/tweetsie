extern crate twitter_api;
extern crate gtk;

mod main_window;
use main_window::MainWindow;

mod tweet_stream;
use tweet_stream::TweetStream;

mod tweet;
use tweet::Tweet;

fn main() {
    if gtk::init().is_err() {
        panic!("failed to initialize gtk");
        return;
    }

    let mut window = MainWindow::new();
    window.show();

    for col in 0..3 {
        let mut testStream = TweetStream::new();
        for x in 0..10 {
            testStream.add_tweet(Tweet::new());
        }

        window.add_stream(testStream);
    }

    gtk::main();
}