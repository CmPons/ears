#[cfg(unix)]
extern crate pkg_config;

#[cfg(unix)]
fn main() {
    for name in ["openal", "sndfile"].iter() {
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
