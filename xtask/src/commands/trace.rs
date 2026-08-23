use std::{
    path::{Path, PathBuf},
    process::Command,
};

use anyhow::{Result, bail};
use which::which;

use crate::utils::print_error;

pub fn trace(file: PathBuf, cwd: PathBuf) -> Result<()> {
    if let Ok(path) = which("surfer") {
        return run_surfer(path, file, cwd);
    }

    let local = PathBuf::from(".surfer/surfer");
    if local.is_file() && is_executable(&local) {
        return run_surfer(local, file, cwd);
    }

    print_error("Error: surfer not found");
    println!("You can install it globally or locally to '.surfer/surfer'.");
    println!("See https://gitlab.com/surfer-project/surfer");

    bail!("surfer not found");
}

fn run_surfer(bin: PathBuf, file: PathBuf, cwd: PathBuf) -> Result<()> {
    let status = Command::new(bin.canonicalize()?)
        .arg(file)
        .current_dir(cwd)
        .status()?;

    if !status.success() {
        bail!("surfer exited with failure");
    }

    Ok(())
}

fn is_executable(p: &Path) -> bool {
    use std::os::unix::fs::PermissionsExt;
    if let Ok(meta) = std::fs::metadata(p) {
        return meta.permissions().mode() & 0o111 != 0;
    }
    false
}
