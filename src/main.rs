extern crate twitter_stream as twitter;
extern crate gtk;
extern crate futures;

mod view;
mod model;

use view::main_window::MainWindow;
use view::tweet_stream::TweetStream;
use view::tweet::Tweet;

use model::TweetsieModel;

fn main() {
    if gtk::init().is_err() {
        panic!("failed to initialize gtk");
        return;
    }

    let model = TweetsieModel::new();

    let mut window = MainWindow::new(&model);
    window.show();

    if !model.is_logged_in() {
        window.login();    
    }

    for _ in 0..3 {
        let mut test_stream = TweetStream::new();
        for _ in 0..10 {
            test_stream.add_tweet(Tweet::new());
        }

        window.add_stream(test_stream);
    }

    gtk::main();
}
