use std::{env, path::PathBuf};

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let mut c_api_path = PathBuf::from(crate_dir.clone());
    c_api_path.push("../c_api");
    c_api_path.push("include");
    c_api_path.push("nv_api");
    c_api_path.set_file_name("nv_api_internal.h");

    // cxx_build::bridge("src/lib.rs")
    //     .file("src/nv_api.cc")
    //     .flag_if_supported("-std=c++14")
    //     .compile("nv_api")
    let _build = cxx_build::bridge("src/lib.rs").flag_if_supported("-std=c++17");
}
