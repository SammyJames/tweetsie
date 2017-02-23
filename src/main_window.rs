use gtk;
use gtk::prelude::*;
use gtk::{Builder, Window, Box};

use TweetStream;

pub struct MainWindow {
    root: Window,
    columns: Box,
}

impl MainWindow {
    pub fn new() -> MainWindow {
        let window_src = include_str!("../res/root.glade");
        let builder = Builder::new_from_string(window_src);

        let root: Window = builder.get_object("Root").unwrap();
        let cols: Box = builder.get_object("ColumnBox").unwrap();

        root.connect_delete_event(|_, _| {
            gtk::main_quit();
            Inhibit(false)
        });

        MainWindow {
            root: root,
            columns: cols,
        }
    }

    pub fn show(&mut self) {
        self.root.show_all();
    }

    pub fn hide(&mut self) {
        unimplemented!();
    }

    pub fn add_stream(&mut self, new_stream: TweetStream) {
        self.columns.add(&new_stream.root);
    }
}
