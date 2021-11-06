use std::process::Command;

use anyhow::{Context, Result};

const LIBSTD: [&str; 3] = ["std", "core", "alloc"];

fn main() -> Result<()> {
    Command::new("./x.py")
        .args(&["doc", "library/std"])
        .status()
        .context("failed to index `set:libstd`")?;

    for krate in LIBSTD {
        let from = format!("build/x86_64-unknown-linux-gnu/doc/{}.json", krate);
        let to = "../crate";
        Command::new("mv")
            .args(&[&from, to])
            .status()
            .with_context(|| format!("failed to store `{}.json` to index", krate))?;
    }

    Ok(())
}
