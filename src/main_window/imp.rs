use gtk::{
    subclass::prelude::*,
    gio::{
        self,
    },
    glib::{
        self,
        subclass::InitializingObject,
    },
    CompositeTemplate,
};

#[derive(CompositeTemplate, Default)]
#[template(resource = "/info/zekefast/FirefoxSessionStoreManager/window.ui")]
pub struct MainWindow;

#[glib::object_subclass]
impl ObjectSubclass for MainWindow {
    const NAME: &'static str = "MainWindow";
    type Type = super::MainWindow;
    type ParentType = gtk::ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for MainWindow {
    fn constructed(&self) {
        // Call "constructed" on parent
        self.parent_constructed();

        // Setup
        let _obj = self.obj();
    }
}

impl WindowImpl for MainWindow {}
impl WidgetImpl for MainWindow {}
impl ApplicationWindowImpl for MainWindow {}