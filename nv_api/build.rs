use std::{env, path::PathBuf};

fn main() {
 
    let _build = cxx_build::bridge("src/lib.rs").flag_if_supported("-std=c++17");
}
