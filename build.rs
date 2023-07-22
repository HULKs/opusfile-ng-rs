use std::{env::var, path::PathBuf};

use bindgen::{Builder, CargoCallbacks};
use pkg_config::probe_library;

fn main() {
    let out_path =
        PathBuf::from(var("OUT_DIR").expect("failed to get OUT_DIR environment variable"));
    let library = probe_library("opusfile").expect("failed to probe library");
    let bindings = Builder::default()
        .header("wrapper.h")
        .clang_args(
            library
                .include_paths
                .iter()
                .map(|include_path| format!("-I{}", include_path.display())),
        )
        .parse_callbacks(Box::new(CargoCallbacks))
        .blocklist_file(".*stdio.h")
        .generate()
        .expect("Failed to generate bindings");

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Failed to write bindings");
}
