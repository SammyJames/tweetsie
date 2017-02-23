use gtk;
use gtk::prelude::*;
use gtk::{Builder, Window, Box};

use view::login_window::LoginWindow;

use TweetStream;
use model::TweetsieModel;

pub struct MainWindow<'a> {
    pub root: Window,
    columns: Box,
    model: &'a TweetsieModel,
}

impl<'a> MainWindow<'a> {
    pub fn new(model: &'a TweetsieModel) -> MainWindow {
        let window_src = include_str!("../../res/root.glade");
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
            model: model,
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

    pub fn login(&mut self) {
        let mut window = LoginWindow::new(&self);
        window.show();
    }
}
