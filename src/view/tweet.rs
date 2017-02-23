
use gtk::{Builder, Box, Label, Image};

pub struct Tweet {
    pub root: Box,
    display_name: Label,
    handle: Label,
    avatar: Image,
    tweet: Label,
}

impl Tweet {
    pub fn new() -> Tweet {
        let tweet_src = include_str!("../../res/tweet.glade");
        let builder = Builder::new_from_string(tweet_src);

        let root: Box = builder.get_object("Root").unwrap();
        let display_name: Label = builder.get_object("DisplayName").unwrap();
        let handle: Label = builder.get_object("Handle").unwrap();
        let tweet: Label = builder.get_object("Tweet").unwrap();
        let avatar: Image = builder.get_object("Avatar").unwrap();

        Tweet {
            root: root,
            display_name: display_name,
            handle: handle,
            tweet: tweet,
            avatar: avatar,
        }
    }
}
