#[cfg(not(target_os = "macos"))]
const LIBFUSE_NAME: &str = "fuse";

#[cfg(target_os = "macos")]
const LIBFUSE_NAME: &str = "osxfuse";

use std::env;

fn main() {
    match env::var("FUSE_CROSS_STATIC_LIB") {
        Ok(val) => {
            println!("cargo:rustc-link-lib=static={}", val);
            match env::var("FUSE_CROSS_STATIC_PATH") {
                Ok(val) => {
                    println!("cargo:rustc-link-search=native={}", val);
                },
                Err(_) => {}
            }
        },
        Err(_) => {
            pkg_config::Config::new()
                .atleast_version("2.6.0")
                .probe(LIBFUSE_NAME)
                .map_err(|e| eprintln!("{}", e))
                .unwrap();
        }
    }
}
