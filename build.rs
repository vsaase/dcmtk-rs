#![allow(non_snake_case)]


extern crate cpp_build;

use cpp_build::Config;

fn main()
{
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/main.rs");
    Config::new().flag("-std=c++11").build("src/main.rs");
    println!("cargo:rustc-link-lib=z");
    println!("cargo:rustc-link-lib=dcmdata");
    println!("cargo:rustc-link-lib=ofstd");
    println!("cargo:rustc-link-lib=oflog");
}

