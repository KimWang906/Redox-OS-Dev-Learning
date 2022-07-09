use std::path::Path;

use anyhow::{Context, Result};
use clap::{App, Arg};

#[path = "../archive_common.rs"]
mod archive_common;
use self::archive_common::{self as archive, Args, DEFAULT_MAX_SIZE};

fn main() -> Result<()> {
    let matches = App::new("redox-initfs-ar")
        .about("create an initfs image from a directory")
        .version(clap::crate_version!())
        .author(clap::crate_authors!())
        .arg(
            Arg::with_name("MAX_SIZE")
                .long("--max-size")
                .short("-m")
                .takes_value(true)
                .required(false)
                .help("Set the upper limit for how large the image can become (default 64 MiB)."),
        )
        .arg(
            Arg::with_name("SOURCE")
                .takes_value(true)
                .required(true)
                .help("Specify the source directory to build the image from."),
        )
        .arg(
            Arg::with_name("OUTPUT")
                .takes_value(true)
                .required(true)
                .long("--output")
                .short("-o")
                .help("Specify the path of the new image file."),
        )
        .get_matches();

    env_logger::init();

    let max_size = if let Some(max_size_str) = matches.value_of("MAX_SIZE") {
        max_size_str
            .parse::<u64>()
            .context("expected an integer for MAX_SIZE")?
    } else {
        DEFAULT_MAX_SIZE
    };

    let source = matches
        .value_of("SOURCE")
        .expect("expected the required arg SOURCE to exist");

    let destination = matches
        .value_of("OUTPUT")
        .expect("expected the required arg OUTPUT to exist");

    let args = Args {
        source: Path::new(source),
        destination_path: Path::new(destination),
        max_size,
    };
    archive::archive(&args)
}
