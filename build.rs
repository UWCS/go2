use std::env;
use std::path::{Path, PathBuf};
use which::which;

// Example custom build script.
fn main() {
    // Tell Cargo that if the given file changes, to rerun this build script.
    println!("cargo:rerun-if-changed=templates/*");
    println!("cargo:rerun-if-changed=static/styles.css");
    println!("cargo:rerun-if-changed=tailwind.config.js");
    if let Some(exe) = find_tailwind() {
        run_tailwind(&exe)
    } else {
        panic!("Tailwind CSS executable not found. Please download tailwindcss and either place it on your PATH or in the root of this Cargo project. https://tailwindcss.com/blog/standalone-cli ")
    }
}

fn find_tailwind() -> Option<PathBuf> {
    which("tailwindcss").ok().or_else(|| {
        let pwd = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
        let path = pwd.join("tailwindcss");
        path.exists().then_some(path.to_path_buf())
    })
}

fn run_tailwind(exe: &Path) {
    let result = std::process::Command::new(exe)
        .arg("-i")
        .arg("static/styles.css")
        .arg("-o")
        .arg("static/output.css")
        .arg("--minify")
        .output()
        .map(|_| ());
    if let Err(e) = result {
        panic!("Failed to run tailwindcss: {}", e)
    }
}
