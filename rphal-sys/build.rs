use std::env;
use std::path::PathBuf;

fn main() {
    let root = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let headers = root.join("headers");
    let hal_include = headers.join("hal/include");
    let wpiutil_include = headers.join("wpiutil/include");

    // Only the macOS universal binaries are vendored so far.
    let platform = match env::var("CARGO_CFG_TARGET_OS").unwrap().as_str() {
        "macos" => "osxuniversal",
        other => panic!("rphal-sys: no vendored HAL libraries for target OS `{other}`"),
    };

    for lib in ["hal", "wpiutil"] {
        let dir = headers.join(lib).join("lib").join(platform);
        println!("cargo:rustc-link-search=native={}", dir.display());
        // Absolute rpath so binaries run straight out of target/.
        println!("cargo:rustc-link-arg=-Wl,-rpath,{}", dir.display());
    }
    println!("cargo:rustc-link-lib=dylib=wpiHal");
    println!("cargo:rustc-link-lib=dylib=wpiutil");

    println!("cargo:rerun-if-changed=wrapper.h");
    println!("cargo:rerun-if-changed=headers");

    let bindings = bindgen::Builder::default()
        .header(root.join("wrapper.h").to_str().unwrap())
        .clang_args(["-x", "c++", "-std=c++20"])
        .clang_arg(format!("-I{}", hal_include.display()))
        .clang_arg(format!("-I{}", wpiutil_include.display()))
        .allowlist_function("HAL_.*|HALSIM_.*")
        .allowlist_type("HAL_.*|HALSIM_.*")
        .allowlist_var("HAL_.*|HALSIM_.*")
        // HAL enums are plain int32_t on the C side; a newtype avoids UB if
        // the HAL ever hands back a value Rust doesn't know about.
        .default_enum_style(bindgen::EnumVariation::NewType {
            is_bitfield: false,
            is_global: false,
        })
        .prepend_enum_name(false)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("rphal-sys: failed to generate HAL bindings");

    let out = PathBuf::from(env::var("OUT_DIR").unwrap()).join("bindings.rs");
    bindings
        .write_to_file(&out)
        .expect("rphal-sys: failed to write bindings");
}
