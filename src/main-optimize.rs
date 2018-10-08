extern crate failure;
extern crate itertools;

extern crate egypt;

use std::env;
use std::fs;
use std::process;

use itertools::{ Itertools };

use egypt::board::{ Board };
use egypt::util;

fn usage() -> ! {
    eprintln!("Usage: optimize <problem> <solutions>");
    process::exit(1);
}

fn main() -> Result<(), failure::Error> {
    let args: Vec<_> = env::args().collect();
    if args.len() != 3 { usage(); }
    let board: Board = fs::read_to_string(&args[1])?.parse()?;
    let sols: Vec<_> = fs::read_to_string(&args[2])?
        .lines()
        .filter(|line| {
            let line = line.trim();
            if line.is_empty() { return false; }
            if line.starts_with('#') { return false; }
            true
        })
        .map(|line| util::parse_solution(line))
        .collect();

    let mut optimized = vec![];
    for sol in sols {
        let mut opts = util::optimize_solution(&board, &sol);
        optimized.append(&mut opts);
    }

    let results = util::solutions_with_step(&board, &optimized);
    for (sol,step) in results {
        println!("{} # rotate={} step={}",
                 sol.iter().join(" "),
                 sol.len(), step);
        assert!(util::verify_solution(&board, &sol));
    }

    Ok(())
}
