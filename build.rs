// build.rs
use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-search=native=./vendor");
    println!("cargo:rustc-link-lib=static=SimConnect");

    for lib in ["shlwapi", "user32", "Ws2_32"] {
        println!("cargo:rustc-link-lib={}", lib);
    }

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .prepend_enum_name(false)
        .clang_arg("-I./vendor")
        .allowlist_file(".*SimConnect\\.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .clang_args(&["-x", "c++", "-std=c++17"])
        .derive_default(true)
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
