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
use egypt::bfs::{ BfsSolver };

fn usage() -> ! {
    eprintln!("Usage: bfs [max_node_count]");
    process::exit(1);
}

fn main() -> Result<(), failure::Error> {
    const MAX_NODE_COUNT_DEFAULT: u64 = 10_000_000;
    let args: Vec<_> = env::args().collect();
    let max_node_count = match args.len() {
        2 => args[1].parse()?,
        1 => MAX_NODE_COUNT_DEFAULT,
        _ => usage(),
    };

    let mut s = String::new();
    io::stdin().read_to_string(&mut s)?;
    let board = Board::from_str(&s)?;

    let mut solver = BfsSolver::new(max_node_count);
    match solver.solve(&board) {
        Ok(sols) => {
            let sols = util::solutions_with_step(&board, &sols);
            for (sol, step) in sols {
                println!("{} # step={}",
                         sol.iter().join(" "),
                         step);
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
