use std::env;
use std::path::PathBuf;

fn main() {
    let root = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let headers = root.join("headers");
    let hal_include = headers.join("hal/include");
    let wpiutil_include = headers.join("wpiutil/include");

    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    let platform = match (target_os.as_str(), target_arch.as_str()) {
        ("macos", _) => "osxuniversal",
        ("linux", "x86_64") => "linuxx86-64",
        (os, arch) => panic!("rphal-sys: no vendored HAL libraries for target `{os}`/`{arch}`"),
    };

    // hal and wpiutil libs are vendored side by side (not split by crate)
    // because libwpiHal's own rpath is `$ORIGIN`: it looks for libwpiutil
    // right next to itself, and that lookup isn't affected by any rpath
    // we set on the final binary.
    let lib_dir = headers.join("lib").join(platform);
    println!("cargo:rustc-link-search=native={}", lib_dir.display());
    // Absolute rpath so binaries run straight out of target/.
    println!("cargo:rustc-link-arg=-Wl,-rpath,{}", lib_dir.display());
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
