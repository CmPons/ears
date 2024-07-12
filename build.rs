#[cfg(unix)]
extern crate pkg_config;

#[cfg(unix)]
fn main() {
    use std::env;

    let cwd = env::var("CARGO_MANIFEST_DIR").unwrap();
    for name in ["sndfile", "openal"].iter() {
        let searchdir = cwd.clone() + "/third_party/" + name + "/libs";
        println!("cargo:rustc-link-search={}", searchdir);
        println!("cargo:rustc-link-lib=static={}", name);
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
