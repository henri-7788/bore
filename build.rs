use std::env;
use std::path::PathBuf;

fn main() {
    // Only embed manifest on Windows (MSVC linker)
    // Note: This will only work with the MSVC toolchain, which is the default on Windows.
    // If using MinGW/GNU toolchain, the linker will ignore these flags.
    if env::var("CARGO_CFG_TARGET_OS").unwrap() == "windows" {
        let manifest_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap())
            .join("bore.manifest");
        
        println!("cargo:rerun-if-changed={}", manifest_path.display());
        println!("cargo:rustc-link-arg=/MANIFEST:EMBED");
        println!("cargo:rustc-link-arg=/MANIFESTFILE:{}", manifest_path.display());
    }
}

