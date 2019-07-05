#![allow(non_snake_case)]


extern crate cpp_build;

fn main()
{
    println!("cargo:rerun-if-changed=build.rs");
    cpp_build::build("src/main.rs");
    println!("cargo:rustc-link-lib=z");
    println!("cargo:rustc-link-lib=dcmdata");
    println!("cargo:rustc-link-lib=ofstd");
    println!("cargo:rustc-link-lib=oflog");
}

