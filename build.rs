use std::env;
use std::fs;
use std::path::{Path, PathBuf};

fn copy_if_exists(src: &Path, dst: &Path) -> std::io::Result<()> {
    if src.exists() {
        fs::copy(src, dst)?;
    }
    Ok(())
}

fn main() {
    println!("cargo:rerun-if-changed=lib/gsl.dll");
    println!("cargo:rerun-if-changed=lib/gslcblas.dll");

    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR is not set"));
    let profile_dir = out_dir
        .ancestors()
        .nth(3)
        .expect("failed to resolve target profile directory");

    let lib_dir = PathBuf::from("lib");
    let _ = copy_if_exists(&lib_dir.join("gsl.dll"), &profile_dir.join("gsl.dll"));
    let _ = copy_if_exists(
        &lib_dir.join("gslcblas.dll"),
        &profile_dir.join("gslcblas.dll"),
    );
}
