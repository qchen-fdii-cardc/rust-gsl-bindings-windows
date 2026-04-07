use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::SystemTime;

fn copy_if_exists(src: &Path, dst: &Path) -> std::io::Result<()> {
    if src.exists() {
        fs::copy(src, dst)?;
    }
    Ok(())
}

fn collect_files(dir: &Path, out: &mut Vec<PathBuf>) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                collect_files(&path, out);
            } else {
                out.push(path);
            }
        }
    }
}

fn modified_or_epoch(path: &Path) -> SystemTime {
    fs::metadata(path)
        .and_then(|m| m.modified())
        .unwrap_or(SystemTime::UNIX_EPOCH)
}

fn newest_input_time(paths: &[PathBuf]) -> SystemTime {
    let mut newest = SystemTime::UNIX_EPOCH;
    for p in paths {
        let t = modified_or_epoch(p);
        if t > newest {
            newest = t;
        }
    }
    newest
}

fn run_generate_bindings_script() {
    let script = Path::new("scripts/generate-gsl-bindings.ps1");

    let mut cmd = Command::new("pwsh");
    cmd.arg("-NoProfile")
        .arg("-ExecutionPolicy")
        .arg("Bypass")
        .arg("-File")
        .arg(script)
        .current_dir(".");

    let output = cmd.output().unwrap_or_else(|e| {
        panic!("failed to launch pwsh for {}: {e}", script.display());
    });

    if !output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        panic!(
            "binding generation failed.\nstdout:\n{}\nstderr:\n{}",
            stdout, stderr
        );
    }
}

fn maybe_regenerate_bindings() {
    let script = PathBuf::from("scripts/generate-gsl-bindings.ps1");
    let wrapper = PathBuf::from("include/wrapper.h");
    let include_dir = PathBuf::from("include/gsl");
    let output = PathBuf::from("src/gsl_bindings.rs");

    let mut source_inputs = vec![script.clone()];
    collect_files(&include_dir, &mut source_inputs);

    let newest_source_input = newest_input_time(&source_inputs);
    let wrapper_time = modified_or_epoch(&wrapper);
    let output_time = modified_or_epoch(&output);

    // Script logic is two-stage: first rebuild wrapper.h from include/gsl,
    // then generate gsl_bindings.rs from wrapper.h.
    // Re-run script only when this pipeline is stale.
    let need_regen = !wrapper.exists()
        || !output.exists()
        || newest_source_input > wrapper_time
        || wrapper_time > output_time;

    if need_regen {
        println!("cargo:warning=Regenerating gsl bindings...");
        run_generate_bindings_script();
    }
}

fn main() {
    println!("cargo:rerun-if-changed=scripts/generate-gsl-bindings.ps1");
    println!("cargo:rerun-if-changed=include/gsl");
    println!("cargo:rerun-if-changed=lib/gsl.dll");
    println!("cargo:rerun-if-changed=lib/gslcblas.dll");

    maybe_regenerate_bindings();

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
