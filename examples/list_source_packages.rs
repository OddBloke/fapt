use std::env;

use fapt::commands;
use fapt::rfc822::RfcMapExt;
use fapt::system::System;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let args: Vec<String> = env::args().skip(1).collect();

    let src_line = if args.is_empty() {
        "deb-src http://deb.debian.org/debian sid main contrib".to_string()
    } else {
        args.join(" ")
    };

    let mut fapt = System::cache_only()?;
    commands::add_sources_entries_from_str(&mut fapt, src_line)?;
    commands::add_builtin_keys(&mut fapt);
    fapt.update().await?;

    for section in commands::all_blocks(&fapt)? {
        let section = section?;
        let map = section.as_map()?;
        let pkg = map.get_value("Package").one_line_req()?;
        println!("{}", pkg);
    }

    Ok(())
}
