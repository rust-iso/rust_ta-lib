extern crate bindgen;

use cmake;
use std::env;
use std::path::PathBuf;

fn main() {
    // std::fs::copy(
    //     "CMakeLists-ta-lib-0.6.0-rc.1.replace.txt",
    //     "3rdparty/ta-lib/CMakeLists.txt",
    // )
    // .unwrap();
    // std::fs::copy(
    //     "cmake_install.ta-lib-0.6.0-rc.1.replace.cmake",
    //     "3rdparty/ta-lib/cmake_install.cmake",
    // )
    // .unwrap();

    let dst = cmake::Config::new("./3rdparty/ta-lib")
        .define("TA_LIB_VERSION_FULL", "0.6.0.rc.1")
        .build();

    println!(
        "cargo:rustc-link-search=native={}",
        dst.join("lib").display()
    );
    // Tell rustc to use nng static library
    println!("cargo:rustc-link-lib=static=ta_lib");

    // println!("cargo:rustc-link-lib=static=ta_lib");
    // let dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    // println!(
    //     "cargo:rustc-link-search=native={}",
    //     Path::new(&dir).join("./3rdparty/ta-lib/lib/").display()
    // );

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
