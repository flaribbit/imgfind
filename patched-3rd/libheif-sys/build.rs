fn main() {
    if std::env::var("DOCS_RS").is_ok() {
        // Don't link with libheif in case of building documentation for docs.rs.
        println!("cargo:rustc-cfg=docs_rs");
        return;
    }

    // Tell cargo to tell rustc to link the system heif
    // shared library.
    #[allow(unused_mut)]
    #[allow(unused_variables)]
    let mut include_dirs: Vec<String> = Vec::new();

    #[cfg(not(target_os = "windows"))]
    if let Err(err) = pkg_config::Config::new()
        .atleast_version("1.16")
        .probe("libheif")
    {
        println!("cargo:warning={}", err);
        std::process::exit(1);
    }

    #[cfg(target_os = "windows")]
    {
        let vcpkg_lib = vcpkg::Config::new()
            .emit_includes(true)
            .find_package("libheif");
        match vcpkg_lib {
            Ok(lib) => {
                // https://users.rust-lang.org/t/bindgen-cant-find-included-file/62687
                use walkdir::WalkDir;
                for path in lib.include_paths {
                    for subdir in WalkDir::new(path)
                        .into_iter()
                        .filter_entry(|e| e.file_type().is_dir())
                    {
                        let dir = subdir.unwrap().path().to_string_lossy().to_string();
                        include_dirs.push(format!("--include-directory={}", dir));
                    }
                }
            }
            Err(err) => {
                println!("cargo:warning={}", err);
                std::process::exit(1);
            }
        }
    }

    #[cfg(feature = "use-bindgen")]
    {
        use std::env;
        use std::path::PathBuf;
        // The bindgen::Builder is the main entry point
        // to bindgen, and lets you build up options for
        // the resulting bindings.
        let mut builder = bindgen::Builder::default()
            // The input header we would like to generate
            // bindings for.
            .header("wrapper.h")
            .generate_comments(true)
            .generate_cstr(true)
            .ctypes_prefix("libc")
            .allowlist_function("heif_.*")
            .allowlist_type("heif_.*")
            .size_t_is_usize(true)
            .clang_args([
                "-fparse-all-comments",
                "-fretain-comments-from-system-headers",
            ]);
        if !include_dirs.is_empty() {
            dbg!(&include_dirs);
            builder = builder.clang_args(include_dirs);
        }

        // Finish the builder and generate the bindings.
        let bindings = builder
            .generate()
            // Unwrap the Result and panic on failure.
            .expect("Unable to generate bindings");

        // Write the bindings to the $OUT_DIR/bindings.rs file.
        let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
        bindings
            .write_to_file(out_path.join("bindings.rs"))
            .expect("Couldn't write bindings!");
    }
}
