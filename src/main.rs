mod main_window;

use gtk::{
    self,
    prelude::*,
    ApplicationWindow,
    FileDialog,
    ListStore,
    FileFilter,
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

use main_window::MainWindow;


const APP_ID: &str = "info.zekefast.FirefoxSessionStoreManager";


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

    window.present();
}
