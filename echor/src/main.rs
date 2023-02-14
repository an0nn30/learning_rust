use clap::{App, Arg};

fn main() {
    let matches = App::new("echor")
        .version("0.1.0")
        .author("Dustin M. <example@example.com>")
        .about("Rust echo")
        .arg(
            Arg::with_name("text")
                .value_name("TEXT")
                .help("Input text")
                .required(true)
                .min_values(1),
        )
        .arg(
            Arg::with_name("omit_newline")
                .short("n")
                .help("Do not print newline")
                .takes_value(false),
        )
        .get_matches();

    let text = matches.values_of_lossy("text").unwrap();
    let omit_newline = matches.is_present("omit_newline");

    // Personally, I do not like having expressions embedded in statements like this.
    // I would prefer to have a separate statement for the print!() call and the if statement.
    print!("{}{}", text.join(" "), if omit_newline { "" } else { "\n" });
}
