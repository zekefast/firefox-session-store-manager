mod imp;

use gtk::{
    prelude::*,
    subclass::prelude::*,
    glib::{
        self,
        Object,
    },
    gio::{
        self,
    },
    Application,
};

glib::wrapper! {
    pub struct MainWindow(ObjectSubclass<imp::MainWindow>)
        @extends
            gtk::ApplicationWindow,
            gtk::Window,
            gtk::Widget,
        @implements
            gio::ActionGroup,
            gio::ActionMap,
            gtk::Accessible,
            gtk::Buildable,
            gtk::ConstraintTarget,
            gtk::Native,
            gtk::Root,
            gtk::ShortcutManager;
}

impl MainWindow {
    pub fn new(app: &Application) -> Self {
        Object::builder()
            .property("application", app)
            .build()
    }
}