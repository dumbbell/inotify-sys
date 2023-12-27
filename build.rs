extern crate pkg_config;

use std::env;

fn main() {
    let target_os = env::var_os("CARGO_CFG_TARGET_OS").unwrap();
    if target_os == "freebsd" {
        pkg_config::probe_library("libinotify").unwrap();
    }
}
