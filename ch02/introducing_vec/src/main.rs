use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

use clap::Arg;
use regex::Regex;

struct CmdArgs {
    pattern: String,
    input: String,
}

fn parse_cmd_args() -> CmdArgs {
    let matches = clap::Command::new("grep-lite")
        .version("0.1")
        .about("searches for patterns")
        .arg(
            Arg::new("pattern")
                .help("The pattern to search for")
                .required(true)
                .num_args(1),
        )
        .arg(
            Arg::new("input")
                .help("File to search")
                .required(true)
                .num_args(1),
        )
        .get_matches();

    CmdArgs {
        // `pattern`은 입력되어 있으므로 `unwrap`을 호출해도 문제가 없다.
        pattern: matches.get_one("pattern").cloned().unwrap(),
        // `pattern`은 입력되어 있으므로 `unwrap`을 호출해도 문제가 없다.
        input: matches.get_one("input").cloned().unwrap(),
    }
}

fn process_lines<T: BufRead>(reader: T, re: Regex) {
    reader.lines().for_each(|line_| {
        let line = line_.unwrap();
        if re.find(&line).is_some() {
            println!("{}", line);
        }
    });
}

fn main() {
    let cmd_args = parse_cmd_args();
    let pattern = &cmd_args.pattern;
    let re = Regex::new(pattern).expect("Failed to get new Regex");

    let input = &cmd_args.input;
    if input == "-" {
        let stdin = io::stdin();
        let reader = stdin.lock();
        process_lines(reader, re);
    } else {
        let f = File::open(input).unwrap();
        let reader = BufReader::new(f);
        process_lines(reader, re);
    }
}
