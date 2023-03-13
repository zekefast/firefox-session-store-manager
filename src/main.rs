use gtk::{
    self,
    gio::{self},
    glib::{self, prelude::*},
    prelude::*,
    Application, ApplicationWindow,
};

const APP_ID: &str = "info.zekefast.FirefoxSessionStoreManager";
const TITLE: &str = "Firefox Session Store Manager";
const DEFAULT_WIDTH: i32 = 600;
const DEFAULT_HEIGHT: i32 = 300;

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(build_ui);

    app.run()
}

fn build_ui(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title(TITLE)
        .default_width(DEFAULT_WIDTH)
        .default_height(DEFAULT_HEIGHT)
        .build();

    window.present();
}
