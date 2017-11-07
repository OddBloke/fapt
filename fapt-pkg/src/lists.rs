use release::ReleaseContent;
use Hashes;

use errors::*;

#[derive(Debug)]
enum Compression {
    None,
    Gz,
}

impl Compression {
    fn suffix(&self) -> &'static str {
        use self::Compression::*;
        match *self {
            None => "",
            Gz => ".gz",
        }
    }
}

#[derive(Debug)]
pub struct List {
    pub path: String,
    codec: Compression,
    compressed_hashes: Hashes,
    decompressed_hashes: Hashes,
}

impl List {
    pub fn local_name(&self) -> String {
        use ::hex::ToHex;
        self.decompressed_hashes.sha256.to_hex()
    }
}

pub fn find_file(contents: &[ReleaseContent], base: &str) -> Result<List> {

    let gz_name = format!("{}{}", base, Compression::Gz.suffix());

    let mut gz_hashes = None;
    let mut raw_hashes = None;

    for content in contents {
        if content.name == base {
            raw_hashes = Some(content.hashes);
        } else if content.name == gz_name {
            gz_hashes = Some(content.hashes);
        }
    }

    let raw_hashes = raw_hashes.ok_or("file not found in release")?;

    Ok(match gz_hashes {
        Some(gz_hashes) => List {
            path: gz_name,
            codec: Compression::Gz,
            compressed_hashes: gz_hashes,
            decompressed_hashes: raw_hashes,
        },
        None => List {
            path: base.to_string(),
            codec: Compression::None,
            compressed_hashes: raw_hashes,
            decompressed_hashes: raw_hashes,
        },
    })
}
