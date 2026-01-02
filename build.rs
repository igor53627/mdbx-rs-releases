use std::env;
use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;

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
    let version = env!("CARGO_PKG_VERSION");

    let lib_dir = out_dir.join("lib");
    fs::create_dir_all(&lib_dir).expect("Failed to create lib directory");

    // Check if already built
    let lib_path = lib_dir.join("libmdbx_rs.a");
    if lib_path.exists() {
        println!("cargo:rustc-link-search=native={}", lib_dir.display());
        println!("cargo:rustc-link-lib=static=mdbx_rs");
        println!("cargo:rerun-if-env-changed=MDBX_RS_LIB_DIR");
        return;
    }

    // Download and build from source
    let source_url = format!(
        "https://github.com/igor53627/mdbx-rs-releases/releases/download/v{}/mdbx-rs-source.tar.gz",
        version
    );

    eprintln!("Downloading mdbx-rs source from {}", source_url);

    let response = reqwest::blocking::get(&source_url)
        .unwrap_or_else(|e| panic!("Failed to download from {}: {}", source_url, e));

    if !response.status().is_success() {
        panic!(
            "Failed to download {}: HTTP {}. Make sure v{} is released with source tarball.",
            source_url,
            response.status(),
            version
        );
    }

    let bytes = response.bytes().expect("Failed to read response");

    // Extract tarball
    let source_dir = out_dir.join("source");
    fs::create_dir_all(&source_dir).expect("Failed to create source directory");
    
    let decoder = flate2::read::GzDecoder::new(&bytes[..]);
    let mut archive = tar::Archive::new(decoder);
    archive.unpack(&source_dir).expect("Failed to extract tarball");

    // Find the extracted directory (mdbx-rs-source-VERSION)
    let source_subdir = fs::read_dir(&source_dir)
        .expect("Failed to read source dir")
        .filter_map(|e| e.ok())
        .find(|e| e.file_name().to_string_lossy().starts_with("mdbx-rs-source"))
        .expect("Source directory not found in tarball")
        .path();

    eprintln!("Building mdbx-rs from source in {:?}", source_subdir);

    // Build with cargo
    let status = Command::new("cargo")
        .current_dir(&source_subdir)
        .env("CARGO_TARGET_DIR", out_dir.join("target"))
        .args([
            "build",
            "--release",
            "-p", "mdbx-rs",
            "--target", &target,
        ])
        .status()
        .expect("Failed to run cargo build");

    if !status.success() {
        panic!("cargo build failed with status: {}", status);
    }

    // Copy the built library
    let built_lib = out_dir
        .join("target")
        .join(&target)
        .join("release")
        .join("libmdbx_rs.a");

    if !built_lib.exists() {
        // Try without target subdirectory
        let alt_lib = out_dir.join("target").join("release").join("libmdbx_rs.a");
        if alt_lib.exists() {
            fs::copy(&alt_lib, &lib_path).expect("Failed to copy library");
        } else {
            panic!(
                "Built library not found at {:?} or {:?}",
                built_lib, alt_lib
            );
        }
    } else {
        fs::copy(&built_lib, &lib_path).expect("Failed to copy library");
    }

    eprintln!("Successfully built mdbx-rs from source");

    println!("cargo:rustc-link-search=native={}", lib_dir.display());
    println!("cargo:rustc-link-lib=static=mdbx_rs");
    println!("cargo:rerun-if-env-changed=MDBX_RS_LIB_DIR");
}
