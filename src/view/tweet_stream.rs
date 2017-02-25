use gtk::prelude::*;
use gtk::{Builder, Box, Label};

use view::tweet::Tweet;

pub struct TweetStream {
    pub root: Box,
    pub children: Box,
    title: Label,
}

impl TweetStream {
    pub fn new() -> TweetStream {

        let stream_src = include_str!("../../res/stream.glade");
        let builder = Builder::new_from_string(stream_src);

        let root: Box = builder.get_object("Root").unwrap();
        let children: Box = builder.get_object("Children").unwrap();
        let title: Label = builder.get_object("Title").unwrap();

        TweetStream {
            root: root,
            children: children,
            title: title,
        }
    }

    pub fn add_tweet(&mut self, tweet: Tweet) {
        self.children.add(&tweet.root);
    }

    pub fn set_name(&mut self, name: String) {
        self.title.set_text(&name);
    }
}
