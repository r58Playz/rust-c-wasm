use std::{
    env,
    path::PathBuf,
    process::{Command, ExitStatus},
};

use anyhow::{anyhow, bail, Context, Result};

trait ExitStatusExt {
    fn was_ok(&self) -> Result<()>;
}

impl ExitStatusExt for ExitStatus {
    fn was_ok(&self) -> Result<()> {
        if self.success() {
            Ok(())
        } else {
            bail!("exit status was not ok");
        }
    }
}

fn main() -> Result<()> {
    let clang_path = which::which("clang").context("failed to find clang")?;
    let llvm_ar_path = which::which("llvm-ar").context("failed to find llvm-ar")?;
    let out_path: PathBuf =
        (env::var("OUT_DIR").context("no OUT_DIR variable found")? + "/libtest1/").into();

    if std::fs::exists(&out_path).context("failed to check if out_path exists")? {
        std::fs::remove_dir_all(&out_path).context("failed to remove out_path")?;
    }
    std::fs::create_dir_all(&out_path).context("failed to create out_path")?;

    println!("cargo:warning=out_path: {:?}", out_path);

    Command::new(&clang_path)
        .args(["--target=wasm32-unknown-unknown", "-c", "test.c", "-o"])
        .arg(out_path.join("libtest1.o"))
        .spawn()
        .context("clang failed")?
        .wait()
        .context("clang failed")?
        .was_ok()
        .context("clang failed")?;

    Command::new(&llvm_ar_path)
        .arg("rcs")
        .arg(out_path.join("libtest1.a"))
        .arg(out_path.join("libtest1.o"))
        .spawn()
        .context("llvm-ar failed")?
        .wait()
        .context("llvm-ar failed")?
        .was_ok()
        .context("llvm-ar failed")?;

    println!("cargo:rustc-link-lib=test1");
    println!(
        "cargo:rustc-link-search={}",
        out_path
            .into_os_string()
            .into_string()
            .map_err(|_| anyhow!("failed to get path string"))?
    );

    Ok(())
}
