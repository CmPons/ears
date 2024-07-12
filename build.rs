#[cfg(unix)]
extern crate pkg_config;

#[cfg(unix)]
fn main() {
    use std::env;

    // HACK to just get everything building
    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_default();
    if target_arch.as_str() == "wasm32" {
        return;
    }

    let cwd = env::var("CARGO_MANIFEST_DIR").unwrap();
    let sndfiledir = cwd + "/third_party/sndfile/libs";
    println!("cargo:rustc-link-search={}", sndfiledir);
    println!("cargo:rustc-link-lib=static=sndfile");

    for name in ["openal"].iter() {
        let lib = pkg_config::Config::new()
            .print_system_libs(false)
            .find(name)
            .unwrap();

        for include in lib.include_paths.iter() {
            println!("cargo:include={}", include.display());
        }

        for link in lib.link_paths.iter() {
            println!("cargo:rustc-link-search=native={}", link.display());
        }
    }
}

#[cfg(all(windows, target_arch = "x86"))]
fn main() {
    println!("cargo:rustc-link-search=native=C:\\msys32\\mingw64\\lib");
}

#[cfg(all(windows, target_arch = "x86_64"))]
fn main() {
    println!("cargo:rustc-link-search=native=C:\\msys64\\mingw64\\lib");
}

#[cfg(all(not(windows), not(unix)))]
fn main() {}
