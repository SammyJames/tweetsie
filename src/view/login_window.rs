use gtk;
use gtk::prelude::*;
use gtk::{Builder, Window};

use view::MainWindowView;

#[derive(Debug)]
pub struct LoginWindow {
    root: Window,
}

impl LoginWindow {
    pub fn new(window: &MainWindowView) -> LoginWindow {
        let window_src = include_str!("../../res/login.glade");
        let builder = Builder::new_from_string(window_src);

        let root: Window = builder.get_object("Root").unwrap();

        root.connect_delete_event(|_, _| Inhibit(false));

        let parent: Option<&Window> = Some(&window.root);
        root.set_transient_for(parent);
        root.set_modal(true);

        LoginWindow { root: root }
    }


    pub fn show(&mut self) {
    	self.root.present();
    }
}
