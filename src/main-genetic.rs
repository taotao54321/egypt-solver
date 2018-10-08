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
use egypt::genetic::{ GeneticSolver };

fn usage() -> ! {
    eprintln!("Usage: genetic <max_len> [n_generation]");
    process::exit(1);
}

fn main() -> Result<(), failure::Error> {
    const N_GENE_DEFAULT: u32 = 1000;
    let args: Vec<_> = env::args().collect();
    let (max_len, n_gene) = match args.len() {
        2 => (args[1].parse()?, N_GENE_DEFAULT),
        3 => (args[1].parse()?, args[2].parse()?),
        _ => usage(),
    };

    let mut s = String::new();
    io::stdin().read_to_string(&mut s)?;

    let board = Board::from_str(&s)?;

    let mut solver = GeneticSolver::new(max_len, n_gene);
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
