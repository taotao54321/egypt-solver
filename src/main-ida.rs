extern crate failure;
extern crate itertools;

extern crate egypt;

use std::env;
use std::io::{ self, prelude::* };
use std::process;
use std::str::{ FromStr };

use itertools::{ Itertools };

use egypt::board::{ Board };
use egypt::util;

use egypt::{ Solver };
use egypt::ida::{ IdaSolver };

fn usage() -> ! {
    eprintln!("Usage: ida [max_depth_ini] [max_node_count]");
    process::exit(1);
}

fn main() -> Result<(), failure::Error> {
    const MAX_DEPTH_INI_DEFAULT:  u32 = 0;
    const MAX_NODE_COUNT_DEFAULT: u64 = 1_000_000_000_000_000_000;
    let args: Vec<_> = env::args().collect();
    let (max_depth_ini, max_node_count) = match args.len() {
        3 => (args[1].parse()?, args[2].parse()?),
        2 => (args[1].parse()?, MAX_NODE_COUNT_DEFAULT),
        1 => (MAX_DEPTH_INI_DEFAULT, MAX_NODE_COUNT_DEFAULT),
        _ => usage(),
    };

    let mut s = String::new();
    io::stdin().read_to_string(&mut s)?;
    let board = Board::from_str(&s)?;

    let mut solver = IdaSolver::new(max_depth_ini, max_node_count);
    match solver.solve(&board) {
        Ok(sols) => {
            let sols = util::solutions_with_step(&board, &sols);
            for (sol, step) in sols {
                println!("{} # rotate={} step={}",
                         sol.iter().join(" "),
                         sol.len(), step);
                assert!(util::verify_solution(&board, &sol));
            }
            Ok(())
        },
        Err(e) => {
            eprintln!("Too many nodes. solutions:");
            eprintln!("{:?}", e.solutions());
            Err(e.into())
        }
    }
}
