use std::fs;
use std::io;
use std::path::Path;
use std::path::PathBuf;

use reqwest;
use reqwest::header;
use reqwest::header::IfModifiedSince;

use tempfile_fast::persistable_tempfile_in;

use errors::*;

pub struct Download {
    from: reqwest::Url,
    to: PathBuf,
}

impl Download {
    pub fn from_to<P: AsRef<Path>>(from: reqwest::Url, to: P) -> Self {
        Download {
            from,
            to: to.as_ref().to_path_buf(),
        }
    }
}

pub fn fetch(client: &reqwest::Client, downloads: &[Download]) -> Result<()> {
    // TODO: reqwest parallel API, when it's stable

    for download in downloads {
        use std::io::Write;
        print!("Downloading: {} ... ", download.from);
        io::stdout().flush()?;
        fetch_single(client, download).chain_err(|| {
            format!("downloading {} to {:?}", download.from, download.to)
        })?;
    }

    Ok(())
}

fn fetch_single(client: &reqwest::Client, download: &Download) -> Result<()> {

    let mut req = client.get(download.from.as_ref());

    if download.to.exists() {
        req.header(IfModifiedSince(download.to.metadata()?.modified()?.into()));
    }

    let mut resp = req.send().chain_err(|| "initiating request")?;

    let status = resp.status();
    if reqwest::StatusCode::NotModified == status {
        println!("already up to date.");
        return Ok(());
    } else if !status.is_success() {
        bail!(
            "couldn't download {}: server responded with {:?}",
            download.from,
            status
        );
    }
    let parent = download.to.parent().ok_or("path must have parent")?;

    fs::create_dir_all(parent).chain_err(|| {
        format!("creating directories: {:?}", parent)
    })?;

    let mut tmp = persistable_tempfile_in(parent).chain_err(
        || "couldn't create temporary file",
    )?;

    if let Some(len) = resp.headers().get::<header::ContentLength>() {
        tmp.set_len(**len).chain_err(
            || "pretending to allocate space",
        )?;
    }

    io::copy(&mut resp, tmp.as_mut()).chain_err(
        || "copying data",
    )?;

    if download.to.exists() {
        fs::remove_file(&download.to).chain_err(
            || "removing destination for overwriting",
        )?;
    }

    tmp.persist_noclobber(&download.to).chain_err(
        || "persisting result",
    )?;

    // TODO: move the modification time back to the actual server claimed time

    println!("complete.");

    Ok(())
}
