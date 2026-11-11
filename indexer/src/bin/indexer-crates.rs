use std::{env::temp_dir, path::PathBuf};

use anyhow::Result;
use crates_io_api::{AsyncClient, Crate, CratesPage, CratesQueryBuilder, Sort};

use tokio::{fs::OpenOptions, io::copy, process::Command};

async fn index_krate(krate: &Crate) -> Result<()> {
    let temp = temp_dir();
    let path = temp.join(format!("{}.tar.gz", krate.name));
    let url = format!(
        "https://static.crates.io/crates/{name}/{name}-{version}.crate",
        name = krate.name,
        version = krate.max_version,
    );

    let resp = reqwest::get(url).await?;
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(path)
        .await?;

    copy(&mut resp.bytes().await?.as_ref(), &mut file).await?;

    Command::new("tar")
        .args(&["-xf", &format!("{}.tar.gz", krate.name)])
        .current_dir(&temp)
        .status()
        .await?;

    let unpacked = temp.join(format!("{}-{}", krate.name, krate.max_version));
    let cargo = Command::new("cargo")
        .args(&["+nightly", "rustdoc"])
        .env("RUSTDOCFLAGS", "--output-format=json -Z unstable-options")
        .current_dir(&unpacked)
        .status()
        .await?;

    let mv = Command::new("mv")
        .args(&[
            unpacked.join(format!("target/doc/{}.json", krate.name)),
            PathBuf::from("crate"),
        ])
        .status()
        .await?;

    for status in [cargo, mv] {
        if !status.success() {
            return Err(anyhow::anyhow!("one of the commands failed!"));
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let client = AsyncClient::new(
        "roogle (git@hkmatsumoto.com)",
        std::time::Duration::from_millis(1000),
    )?;

    let CratesPage { crates: krates, .. } = client
        .crates(
            CratesQueryBuilder::new()
                .sort(Sort::Downloads)
                .page_size(100)
                .build(),
        )
        .await?;
    let mut json = vec![];
    for krate in krates {
        if index_krate(&krate).await.is_ok() {
            json.push(krate.name);
        }
    }

    let json = serde_json::to_string(&json)?;
    std::fs::write("set/crates.json", &json)?;

    Ok(())
}
