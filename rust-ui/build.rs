use std::env;
use std::path::PathBuf;

fn main() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let cpp_build_dir = manifest_dir.join("..").join("cpp-core").join("build");

    println!("cargo:rustc-link-search=native={}", cpp_build_dir.display());
    println!("cargo:rustc-link-lib=dylib=keyboard_core");
}
