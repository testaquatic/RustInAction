use clap::{Arg, Command};
use rayon::prelude::*;

use crate::draw_svg::HEIGHT;

pub struct CmdArgs {
    pub input: String,
    pub save_to: String,
}

impl CmdArgs {
    pub fn get() -> CmdArgs {
        let matches = Command::new("render-hex")
            .arg(Arg::new("input").required(true))
            .arg(Arg::new("save_to"))
            .get_matches();

        let input = matches.get_one::<String>("input").unwrap().to_string();
        let save_to = match matches.get_one::<String>("save_to") {
            None => format!("{}.svg", input),
            Some(path) => path.to_string(),
        };

        CmdArgs { input, save_to }
    }
}

pub enum Operation {
    Forward(isize),
    TurnLeft,
    TurnRight,
    Home,
    Noop(u8),
}

pub fn parse(input: &str) -> Vec<Operation> {
    input
        .par_bytes()
        .map(|byte| match byte {
            b'0' => Operation::Home,
            b'1'..=b'9' => {
                let distance = (byte - 0x30) as isize;
                Operation::Forward(distance * (HEIGHT / 10))
            }
            b'a' | b'b' | b'c' => Operation::TurnLeft,
            b'd' | b'e' | b'f' => Operation::TurnRight,
            _ => Operation::Noop(byte),
        })
        .collect()
}
