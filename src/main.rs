//! Bricktools
//!
//! A small set of tools to manipulate Bricklink wanted lists and perform
//! price analysis
use std::error;
use std::io::{Error as IOError, ErrorKind};

use bricktools::merge;

use clap::{App, Arg};

/// CLI Tooling

fn main() -> Result<(), Box<dyn error::Error>> {
    let commands = App::new("Bricktools")
        .version("0.1")
        .author("Rob Story")
        .about("Bricklink wanted list helper tools")
        .subcommand(
            App::new("merge")
                .about("Merges two Bricklink wanted lists")
                .arg(
                    Arg::with_name("left")
                        .short('l')
                        .required(true)
                        .takes_value(true)
                        .about("Path to lefthand wanted list, will have right merged into it"),
                )
                .arg(
                    Arg::with_name("right")
                        .short('r')
                        .required(true)
                        .takes_value(true)
                        .about("Path to righthand wanted list, will be merged into left"),
                )
                .arg(
                    Arg::with_name("output")
                        .short('o')
                        .required(true)
                        .takes_value(true)
                        .about("Path to merged output file"),
                ),
        )
        .get_matches();

    match commands.subcommand() {
        ("merge", Some(merge_args)) => merge(merge_args),
        _ => Err(Box::new(IOError::new(
            ErrorKind::InvalidInput,
            "Invalid command input",
        ))),
    }
}
