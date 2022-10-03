use gtk::gio;

fn main() {
    gio::compile_resources(
        "resources",
        "resources/gresources.xml",
        "ygod.gresource",
    );
}
