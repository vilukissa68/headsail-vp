//! This Cargo build script finds the linker on setups where using a .cargo/config.toml file would
//! be inconvenient, e.g. in a Cargo workspace.

use std::{env, fs, path};

fn main() {
    // Put link script in our output directory and ensure it's on the linker search path
    let out = &path::PathBuf::from(env::var_os("OUT_DIR").unwrap());
    fs::copy("mem_hpc.x", out.join("mem_hpc.x")).unwrap();
    if cfg!(feature = "hpc") {
        if cfg!(feature = "sdram") {
            println!("cargo:rustc-env=RUSTFLAGS=-C link-arg=-Tsdram_hpc.x -C link-arg=-Tlink.x");
        } else {
            println!("cargo:rustc-env=RUSTFLAGS=-C link-arg=-Tmem_hpc.x -C link-arg=-Tlink.x");
        }
    }
    fs::copy("sdram_hpc.x", out.join("sdram_hpc.x")).unwrap();
    fs::copy("mem_sysctrl.x", out.join("mem_sysctrl.x")).unwrap();
    println!("cargo:rustc-link-search={}", out.display());
}
