use std::env;
use std::fs::{self, File};

use std::path::PathBuf;

fn main() {
    // Allow using a locally built library (for offline builds or custom builds)
    if let Ok(lib_dir) = env::var("MDBX_RS_LIB_DIR") {
        println!("cargo:rustc-link-search=native={}", lib_dir);
        println!("cargo:rustc-link-lib=static=mdbx_rs");
        println!("cargo:rerun-if-env-changed=MDBX_RS_LIB_DIR");
        return;
    }
    
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let target = env::var("TARGET").unwrap();
    
    // Determine artifact name based on target
    let artifact_name = match target.as_str() {
        t if t.contains("x86_64") && t.contains("linux") => "mdbx-rs-linux-x86_64",
        t if t.contains("aarch64") && t.contains("darwin") => "mdbx-rs-macos-aarch64",
        t if t.contains("x86_64") && t.contains("darwin") => "mdbx-rs-macos-x86_64",
        t if t.contains("aarch64") && t.contains("linux") => "mdbx-rs-linux-aarch64",
        _ => {
            panic!(
                "Unsupported target: {}. Supported: x86_64-linux, aarch64-darwin, x86_64-darwin, aarch64-linux",
                target
            );
        }
    };
    
    let version = env!("CARGO_PKG_VERSION");
    let url = format!(
        "https://github.com/igor53627/mdbx-rs-releases/releases/download/v{}/{}.tar.gz",
        version, artifact_name
    );
    
    let lib_dir = out_dir.join("lib");
    fs::create_dir_all(&lib_dir).expect("Failed to create lib directory");
    
    // Check if already downloaded
    let marker = lib_dir.join(".downloaded");
    if marker.exists() {
        println!("cargo:rustc-link-search=native={}", lib_dir.display());
        println!("cargo:rustc-link-lib=static=mdbx_rs");
        println!("cargo:rerun-if-env-changed=MDBX_RS_LIB_DIR");
        return;
    }
    
    eprintln!("Downloading prebuilt mdbx-rs from {}", url);
    
    // Download the tarball
    let response = reqwest::blocking::get(&url)
        .expect(&format!("Failed to download from {}", url));
    
    if !response.status().is_success() {
        panic!(
            "Failed to download {}: HTTP {}. Make sure v{} is released.",
            url, response.status(), version
        );
    }
    
    let bytes = response.bytes().expect("Failed to read response");
    
    // Extract tarball
    let decoder = flate2::read::GzDecoder::new(&bytes[..]);
    let mut archive = tar::Archive::new(decoder);
    archive.unpack(&lib_dir).expect("Failed to extract tarball");
    
    // Create marker file
    File::create(&marker).expect("Failed to create marker");
    
    println!("cargo:rustc-link-search=native={}", lib_dir.display());
    println!("cargo:rustc-link-lib=static=mdbx_rs");
    
    println!("cargo:rerun-if-env-changed=MDBX_RS_LIB_DIR");
}
