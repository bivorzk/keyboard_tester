use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let cpp_build_dir = manifest_dir.join("..").join("cpp-core").join("build");
    let dll = cpp_build_dir.join("keyboard_core.dll");
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let profile_dir = out_dir.ancestors().nth(3).unwrap();

    println!("cargo:rustc-link-search=native={}", cpp_build_dir.display());
    println!("cargo:rustc-link-lib=dylib=keyboard_core");
    println!("cargo:rerun-if-changed={}", dll.display());

    fs::copy(&dll, profile_dir.join("keyboard_core.dll"))
        .expect("failed to copy keyboard_core.dll next to the Rust executable");
}
