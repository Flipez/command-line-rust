use std::error::Error;
use clap::{App, Arg};
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn run(config: Config) -> MyResult<()> {
    for filename in config.files {
      match open(&filename) {
        Err(err) => eprintln!("{}: {}", filename, err),
        Ok(file) => {
            let mut last_num = 0;
            for (line_num, line_result) in file.lines().enumerate() {
                let line = line_result?;
                if config.number_lines {
                    println!("{:>6}\t{}", line_num + 1, line);
                } else if config.number_nonblank_lines {
                    if !line.is_empty() {
                        last_num += 1;
                        println!("{:>6}\t{}", last_num, line)
                    } else {
                        println!()
                    }
                } else {
                    println!("{}", line);
                }
            }
        }
      }
    }
    Ok(())
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("catr")
        .version("0.1.0")
        .author("Flipez <code@brauser.io>")
        .about("Rust cat")
        .arg(
          Arg::with_name("file")
            .value_name("FILE")
            .help("Input file")
            .multiple(true)
            .default_value("-"),
        )
        .arg(
          Arg::with_name("number")
            .short("n")
            .long("number")
            .help("Number lines")
            .takes_value(false)
            .conflicts_with("number_nonblank"),
        )
        .arg(
          Arg::with_name("number_nonblank")
            .long("number-nonblank")
            .short("b")
            .help("Number nonblank lines")
            .takes_value(false),
        )
        .arg(
          Arg::with_name("version")
            .short("V")
            .long("version")
            .help("Prints version information")
            .takes_value(false),
        )
        .get_matches();

    Ok(Config {
        files: matches.values_of_lossy("file").unwrap(),
        number_lines: matches.is_present("number"),
        number_nonblank_lines: matches.is_present("number_nonblank"),
    })
}

fn  open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}