mod main_window;

use std::path::PathBuf;
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
use const_format::formatcp;

use main_window::MainWindow;


const APP_ID: &str = "info.zekefast.FirefoxSessionStoreManager";


mod file_ext {
    use const_format::concatcp;

    pub const SESSIONSTORE: &str = "jsonlz4";
    pub const SESSIONSTORE_BACKUP: &str = "baklz4";

    pub const WILDCARD_PREFIX: &str = "*";
    pub const SEPARATOR: &str = ".";

    pub const SESSIONSTORE_MASK: &str = concatcp!(WILDCARD_PREFIX, SEPARATOR, SESSIONSTORE);
    pub const SESSIONSTORE_BACKUP_MASK: &str = concatcp!(WILDCARD_PREFIX, SEPARATOR, SESSIONSTORE_BACKUP);
}

mod action {
    use const_format::concatcp;

    pub const SEPARATOR: &str = ".";
    pub const APP_PREFIX: &str = "app";
    pub const WIN_PREFIX: &str = "win";

    pub const OPEN_FILE_DIALOG: &str = "open-file-dialog";
    pub const OPEN_FILE_DIALOG_DETAILED: &str = concatcp!(APP_PREFIX, SEPARATOR, OPEN_FILE_DIALOG);

    pub const LOAD_FILE: &str = "load-file";
    pub const LOAD_FILE_DETAILED: &str = concatcp!(APP_PREFIX, SEPARATOR, LOAD_FILE);

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
    let file_filter_json_and_bak = FileFilter::new();
    file_filter_json_and_bak.set_property(
        "name",
        formatcp!(
            "Session alike files ({}, {})",
            file_ext::SESSIONSTORE_MASK,
            file_ext::SESSIONSTORE_BACKUP_MASK,
        ),
    );
    file_filter_json_and_bak.add_suffix(file_ext::SESSIONSTORE);
    file_filter_json_and_bak.add_suffix(file_ext::SESSIONSTORE_BACKUP);

    let file_filter_json = FileFilter::new();
    file_filter_json.set_property(
        "name",
        formatcp!("Session files ({})", file_ext::SESSIONSTORE_MASK),
    );
    file_filter_json.add_suffix(file_ext::SESSIONSTORE);

    let file_filter_bak = FileFilter::new();
    file_filter_bak.set_property(
        "name",
        format!("Session backup files ({})", file_ext::SESSIONSTORE_BACKUP_MASK),
    );
    file_filter_bak.add_suffix(file_ext::SESSIONSTORE_BACKUP);

    let file_filter_all = FileFilter::new();
    file_filter_all.set_property("name", "All files");
    file_filter_all.add_pattern("*");

    let filter_model = gio::ListStore::new(FileFilter::static_type());
    filter_model.append(&file_filter_json_and_bak);
    filter_model.append(&file_filter_json);
    filter_model.append(&file_filter_bak);
    filter_model.append(&file_filter_all);

    let open_file_dialog = FileDialog::builder()
        .title("Open Firefox's Session Store File or It's Backup")
        .modal(true)
        .filters(&filter_model)
        .build();

    let window = MainWindow::new(app);

    let open_file_dialog_action = SimpleAction::new(action::OPEN_FILE_DIALOG, None);
    open_file_dialog_action.connect_activate(clone!(@weak window, @strong open_file_dialog => move |_action, _parameter| {
        open_file_dialog.open(Some(&window), Cancellable::NONE, clone!(@weak window => move |result| {
            if let Ok(Some(pathbuf)) = result.as_ref().map(|file| file.path()) {
                WidgetExt::activate_action(&window, action::LOAD_FILE_DETAILED, Some(&pathbuf.to_variant()))
                    .expect("The action does not exist.");
            }
        }));
    }));
    app.add_action(&open_file_dialog_action);

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

    let load_file_action = SimpleAction::new(action::LOAD_FILE, Some(&PathBuf::static_variant_type()));
    load_file_action.connect_activate(move |_action, parameter| {
        let pathbuf = parameter
            .expect("Could not get parameter of 'app.load-file' action.")
            .get::<PathBuf>()
            .expect("The variant needs to be of type `String`.");

        todo!("Load file using {:?} path", pathbuf);
    });
    app.add_action(&load_file_action);


    window.present();
}
