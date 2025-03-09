use std::process::Command;
use cargo_emit::rerun_if_changed;

fn main() {
    if !cfg!(debug_assertions) {
        rerun_if_changed!("ui");
        println!("cargo:rerun-if-changed=ui");
        Command::new("pnpm")
            .arg("build")
            .current_dir("ui")
            .output()
            .expect("Failed to build frontend");
    }
}