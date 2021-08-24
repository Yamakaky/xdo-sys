use std::env;
use std::path::PathBuf;

fn main() {
    cc::Build::new()
        .file("xdotool/xdo.c")
        .file("xdotool/xdo_search.c")
        .compile("xdo");

    let header_file = "xdotool/xdo.h";
    println!("cargo:rustc-link-lib=xdo");
    println!("cargo:rustc-link-lib=X11");
    println!("cargo:rustc-link-lib=Xtst");
    println!("cargo:rustc-link-lib=Xi");
    println!("cargo:rustc-link-lib=xkbcommon");
    println!("cargo:rustc-link-lib=Xinerama");
    println!("cargo:rerun-if-changed={}", header_file);
    let bindings = bindgen::Builder::default()
        .header(header_file)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .allowlist_function("xdo.*")
        .generate()
        .expect("Unable to generate bindings");
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
