fn main() {
    println!("cargo:rustc-link-lib=xdo");

    #[cfg(feature = "static")]
    {
        cc::Build::new()
            .file("xdotool/xdo.c")
            .file("xdotool/xdo_search.c")
            .compile("xdo");
    }

    #[cfg(feature = "generate")]
    {
        println!("cargo:rustc-link-lib=X11");
        println!("cargo:rustc-link-lib=Xtst");
        println!("cargo:rustc-link-lib=Xi");
        println!("cargo:rustc-link-lib=xkbcommon");
        println!("cargo:rustc-link-lib=Xinerama");

        let header_file = "xdotool/xdo.h";
        println!("cargo:rerun-if-changed={}", header_file);
        let bindings = bindgen::Builder::default()
            .header(header_file)
            .parse_callbacks(Box::new(bindgen::CargoCallbacks))
            .allowlist_function("xdo.*")
            .generate()
            .expect("Unable to generate bindings");
        let out_path = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
        bindings
            .write_to_file(out_path.join("bindings.rs"))
            .expect("Couldn't write bindings!");
    }
}
