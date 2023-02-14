use clap::{App, Arg};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblack_lines: bool,
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("catr")
        .version("0.1.0")
        .author("Dustin M. <justanothernerdhere@outlook.com>")
        .arg(
            Arg::with_name("files")
                .help("The files to read")
                .required(true)
                .multiple(true)
                .default_value("-"),
        )
        .arg(
            Arg::with_name("number_lines")
                .help("Number the lines")
                .short("n")
                .long("number")
                .takes_value(false)
                .conflicts_with("number_nonblank_lines"),
        )
        .arg(
            Arg::with_name("number_nonblank_lines")
                .help("Number the non-blank lines")
                .short("b")
                .long("number-nonblank")
                .takes_value(false),
        )
        .get_matches();

    Ok(Config {
        files: matches.values_of_lossy("files").unwrap(),
        number_lines: matches.is_present("number_lines"),
        number_nonblack_lines: matches.is_present("number_nonblank_lines"),
    })
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

pub fn run(config: Config) -> MyResult<()> {
    for filename in config.files {
        match open(&filename) {
            Err(err) => eprintln!("Failed to open {}: {}", filename, err),
            Ok(_) => {
                if config.number_lines {
                    for (i, line) in open(&filename)?.lines().enumerate() {
                        println!("{:>6} {}", i + 1, line?);
                    }
                } else if config.number_nonblack_lines {
                    for (i, line) in open(&filename)?.lines().enumerate() {
                        let line = line?;
                        if !line.is_empty() {
                            println!("{:>6} {}", i + 1, line);
                        } else {
                            println!("{}", line);
                        }
                    }
                } else {
                    for line in open(&filename)?.lines() {
                        println!("{}", line?);
                    }
                }
            }
        }
    }
    Ok(())
}
