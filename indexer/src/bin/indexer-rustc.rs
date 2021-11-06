use std::process::Command;

use anyhow::{Context, Result};
use glob::glob;

fn main() -> Result<()> {
    Command::new("./x.py")
        .args(&["doc", "compiler"])
        .status()
        .context("failed to index `set:rustc`")?;

    let mut krates = vec![];
    for json in glob("build/x86_64-unknown-linux-gnu/compiler-doc/*.json")
        .context("failed to list indexes")?
    {
        let json = json?;
        let krate = json
            .file_stem()
            .with_context(|| format!("failed to get name of `{}`", json.display()))?;
        let krate = krate
            .to_str()
            .with_context(|| format!("failed to get `&str` from `{:?}`", krate))?;
        let from = format!("build/x86_64-unknown-linux-gnu/compiler-doc/{}.json", krate);

        krates.push(krate.to_owned());

        Command::new("mv")
            .args(&[&from, "../crate"])
            .status()
            .with_context(|| format!("failed to store `{}.json` to index", krate))?;
    }

    let json = serde_json::to_string(&krates).context("serializing crates of `set:rustc` failed")?;
    std::fs::write(
        "../set/rustc.json",
        &json,
    )
    .context("writing content to `rustc.json` failed")?;

    Ok(())
}
