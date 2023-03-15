// GOTCHA: The target name has to correspond GRESOURCE_NAME const in "main.rs" file.
const GRESOURCE_TARGET_NAME: &str = "firefox-session-store-manager.gresource";

fn main() {
    glib_build_tools::compile_resources(
        &["resources"],
        "resources/resources.gresource.xml",
        GRESOURCE_TARGET_NAME,
    );
}
