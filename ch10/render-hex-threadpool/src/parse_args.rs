use std::thread;

use clap::{Arg, Command};
use crossbeam::channel::unbounded;

use crate::{draw_svg::HEIGHT, work::Work};

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

#[derive(Clone)]
pub enum Operation {
    Forward(isize),
    TurnLeft,
    TurnRight,
    Home,
    Noop(u8),
}

fn parse_byte(byte: u8) -> Operation {
    match byte {
        b'0' => Operation::Home,
        b'1'..=b'9' => {
            let distance = (byte - 0x30) as isize;
            Operation::Forward(distance * HEIGHT / 10)
        }
        b'a' | b'b' | b'c' => Operation::TurnLeft,
        b'd' | b'e' | b'f' => Operation::TurnRight,
        _ => Operation::Noop(byte),
    }
}

pub fn parse(input: &str) -> Vec<Operation> {
    let n_threads = 2;
    let (todo_tx, todo_rx) = unbounded();
    let (results_tx, results_rx) = unbounded();
    let mut n_bytes = 0;

    input.bytes().enumerate().for_each(|(i, byte)| {
        todo_tx.send(Work::Task((i, byte))).unwrap();
        n_bytes += 1;
    });

    (0..n_threads).for_each(|_| {
        todo_tx.send(Work::Finished).unwrap();
    });

    (0..n_threads)
        .map(|_| (todo_rx.clone(), results_tx.clone()))
        .for_each(|(todo, results)| {
            thread::spawn(move || loop {
                let task = todo.recv();
                let result = match task {
                    Err(_) => break,
                    Ok(Work::Finished) => break,
                    Ok(Work::Task((i, byte))) => (i, parse_byte(byte)),
                };
                results.send(result).unwrap();
            });
        });
    let mut ops = vec![Operation::Noop(0); n_bytes];
    (0..n_bytes)
        .map(|_| results_rx.recv().unwrap())
        .for_each(|(i, op)| ops[i] = op);

    ops
}
