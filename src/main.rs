extern crate gtk;
extern crate futures;
extern crate secstr;
extern crate crypto;
extern crate rand;
extern crate rustc_serialize;
extern crate url;
extern crate time;
extern crate regex;

#[macro_use] 
extern crate hyper;
extern crate hyper_native_tls;

#[macro_use]
extern crate lazy_static;

mod view;
mod model;
mod constants;
mod auth;
mod util;

use view::main_window::MainWindow;
use view::tweet_stream::TweetStream;
use view::tweet::Tweet;

use model::TweetsieModel;

fn main() {
    if gtk::init().is_err() {
        panic!("failed to initialize gtk");
    }

    let model = TweetsieModel::new();

    let mut window = MainWindow::new(&model);
    window.show();

    let auth_util = auth::PinAuth::new();
    //auth_util.authenticate();
    auth_util.get_request_token();

    if !model.is_logged_in() {
        window.login();    
    }

    for index in 0..3 {
        let mut test_stream = TweetStream::new();
        test_stream.set_name(format!("Stream #{}", index));
        for _ in 0..10 {
            test_stream.add_tweet(Tweet::new());
        }

        window.add_stream(test_stream);
    }

    gtk::main();
}
