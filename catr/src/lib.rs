use clap::{App, Arg};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
    line_range: Option<(usize, usize)>,
}

impl Config {
    pub fn new(
        files: Vec<String>,
        number_lines: bool,
        number_nonblank_lines: bool,
        line_range: Option<(usize, usize)>,
    ) -> Self {
        Config {
            files,
            number_lines,
            number_nonblank_lines,
            line_range,
        }
    }
    fn fmt_number_lines(&self, line: &String, line_number: usize) -> String {
        format!("{:6}\t{}", line_number + 1, line)
    }
    fn fmt_number_nonblank_lines(&self, line: &String, last_number: &mut u32) -> String {
        if !line.is_empty() {
            *last_number += 1;
            format!("{:6}\t{}", last_number, line)
        } else {
            String::new()
        }
    }

    pub fn process_flags(&self, file: Box<dyn BufRead>) {
        let mut last_num = 0;
        for (line_num, line_result) in file.lines().enumerate() {
            if let Some((start, end)) = self.line_range {
                if (line_num + 1 ) < start || (line_num + 1) > end {
                    continue;
                }
            }
            let line = line_result.unwrap();
            if self.number_lines {
                println!("{}", self.fmt_number_lines(&line, line_num).as_str())
            } else if self.number_nonblank_lines {
                println!(
                    "{}",
                    self.fmt_number_nonblank_lines(&line, &mut last_num)
                        .as_str()
                );
            } else {
                println!("{}", line);
            }
        }
    }
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
        .arg(
            Arg::with_name("display_nonprinting")
                .help("Display non-printing characters")
                .short("v")
                .long("show-nonprinting")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("line_range")
                .help("Display only the specified line range")
                .short("r")
                .long("line-range")
                .takes_value(true)
                .value_name("start:end"),
        )
        .get_matches();

    Ok(Config::new(
        matches.values_of_lossy("files").unwrap(),
        matches.is_present("number_lines"),
        matches.is_present("number_nonblank_lines"),
        matches.value_of("line_range").map(|range| {
            let mut split = range.split(':');
            let start = split.next().unwrap().parse::<usize>().unwrap();
            let end = split.next().unwrap().parse::<usize>().unwrap();
            (start, end)
        }),
    ))
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

// TODO: Consider using a function to manage the output based on provided flags
// TODO: See about adding paging support
// TODO: Add support for displaying non-printing characters
pub fn run(config: Config) -> MyResult<()> {
    for filename in &config.files {
        match open(&filename) {
            Err(err) => eprintln!("Failed to open {}: {}", filename, err),
            Ok(file) => config.process_flags(file),
        }
    }
    Ok(())
}
