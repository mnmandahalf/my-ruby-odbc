use std::env;
use std::process::Command;

fn main() {
    let ruby_libdir = Command::new("ruby")
        .args(["-e", "print RbConfig::CONFIG['libdir']"])
        .output()
        .expect("Failed to get Ruby libdir");

    let ruby_libdir = String::from_utf8_lossy(&ruby_libdir.stdout);

    // Add the Ruby version to the library search path
    println!("cargo:rustc-link-search={}", ruby_libdir);

    // Link against the Ruby library
    println!("cargo:rustc-link-lib=ruby");

    println!("cargo:rerun-if-changed=build.rs");
}
