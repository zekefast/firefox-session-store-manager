mod main_window;

use gtk::{
    self,
    prelude::*,
    ApplicationWindow,
    gio::{
        self,
        Cancellable,
        SimpleAction
    },
    glib::{
        self,
        prelude::*,
        clone,
    },
    subclass::prelude::ObjectSubclassIsExt,
};

use adw::{
    self,
    prelude::*,
    Application,
};
use const_format::formatcp;

use main_window::MainWindow;


const APP_ID: &str = "info.zekefast.FirefoxSessionStoreManager";


mod action {
    use const_format::concatcp;

    pub const SEPARATOR: &str = ".";
    pub const APP_PREFIX: &str = "app";
    pub const WIN_PREFIX: &str = "win";

    pub const OPEN_ABOUT_WINDOW: &str = "open-about-window";
    pub const OPEN_ABOUT_WINDOW_DETAILED: &str = concatcp!(APP_PREFIX, SEPARATOR, OPEN_ABOUT_WINDOW);

    pub const OPEN_PREFERENCES_WINDOW: &str = "open-preferences-window";
    pub const OPEN_PREFERENCES_WINDOW_DETAILED: &str = concatcp!(APP_PREFIX, SEPARATOR, OPEN_PREFERENCES_WINDOW);
}

fn main() -> glib::ExitCode {
    // GOTCHA: The name has to correspond to "target" argument given to
    //   "glib_build_tools::compile_resources" function in "build.rs" file.
    gio::resources_register_include!("firefox-session-store-manager.gresource")
        .expect("Failed to register resrouces.");

    let app = Application::builder()
        .application_id(APP_ID)
        .build();

    app.connect_activate(build_ui);

    app.run()
}

fn build_ui(app: &Application) {
    let window = MainWindow::new(app);

    let open_about_window_action = SimpleAction::new(action::OPEN_ABOUT_WINDOW, None);
    open_about_window_action.connect_activate(move |_action, _parameter| {
        todo!("Open About Dialog Window here");
    });
    app.add_action(&open_about_window_action);

    let open_preferences_window_action = SimpleAction::new(action::OPEN_PREFERENCES_WINDOW, None);
    open_preferences_window_action.connect_activate(move |_action, _parameter| {
        todo!("Open Preferences Dialog Window here");
    });
    app.add_action(&open_preferences_window_action);



    window.present();
}
