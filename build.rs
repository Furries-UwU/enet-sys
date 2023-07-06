extern crate bindgen;
extern crate cmake;

use cmake::Config;
use std::env;
use std::path::PathBuf;

fn main() {
    let target = env::var("TARGET").unwrap();
    let is_debug = env::var("DEBUG").unwrap() == "true";

    println!("cargo:rerun-if-changed=wrapper.h");

    let bindings = bindgen::Builder::default()
        .clang_arg("-Ivendor/enet/include/")
        .header("wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .derive_debug(false)
        .blocklist_type("ENetPacket")
        .blocklist_type("_ENetPacket")
        .blocklist_type("_?P?IMAGE_TLS_DIRECTORY.*")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    let dst = Config::new("vendor/enet").build_target("enet").build();

    if target.contains("windows") {
        if is_debug {
            println!(
                "cargo:rustc-link-search=native={}/build/Debug",
                dst.display()
            );
        } else {
            println!(
                "cargo:rustc-link-search=native={}/build/Release",
                dst.display()
            );
        }
        println!("cargo:rustc-link-lib=dylib=winmm");
    } else {
        println!("cargo:rustc-link-search=native={}/build", dst.display());
    }

    println!("cargo:rustc-link-lib=static=enet");
}
