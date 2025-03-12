use std::env;
use std::process::Command;

fn main() {
    let ruby = env::var("RUBY").unwrap_or_else(|_| "ruby".to_string());

    let libruby_output = Command::new(&ruby)
        .args(&["-rrbconfig", "-e", "print RbConfig::CONFIG['LIBRUBY']"])
        .output()
        .expect("failed to get LIBRUBY");
    let libruby_raw = String::from_utf8_lossy(&libruby_output.stdout);
    // "libruby.3.2.dylib"

    let libruby = libruby_raw
        .strip_prefix("lib")
        .unwrap_or(&libruby_raw)
        .trim_end_matches(".dylib")
        .trim_end_matches(".so");

    //  "ruby.3.2"
    println!("cargo:rustc-link-lib=dylib={}", libruby);

    let libdir_output = Command::new(&ruby)
        .args(&["-rrbconfig", "-e", "print RbConfig::CONFIG['libdir']"])
        .output()
        .expect("failed to get libdir");
    let libdir = String::from_utf8_lossy(&libdir_output.stdout);
    println!("cargo:rustc-link-search=native={}", libdir);
}
