use glib_build_tools::compile_resources;

fn main() {
    compile_resources(
        "resources",
        "resources/gresources.xml",
        "compiled.gresource",
    );
}
