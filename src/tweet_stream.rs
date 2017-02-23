use gtk;
use gtk::prelude::*;
use gtk::{Builder, Window, Button, Box, ListBox, ScrolledWindow};

use Tweet;

pub struct TweetStream {
    pub root: ScrolledWindow,
    pub children: Box,
}

impl TweetStream {
    pub fn new() -> TweetStream {

        let stream_src = include_str!("../res/stream.glade");
        let builder = Builder::new_from_string(stream_src);

        let root: ScrolledWindow = builder.get_object("Root").unwrap();
        let children: Box = builder.get_object("Children").unwrap();

        TweetStream {
            root: root,
            children: children,
        }
    }

    pub fn add_tweet(&mut self, tweet: Tweet) {
        self.children.add(&tweet.root);
    }
}