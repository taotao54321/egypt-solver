use std::io::{ self, prelude::* };
use std::time;

use itertools::{ Itertools };

use ::{ Solver, SolverError };
use board::{ Board };
use util;

pub struct IdaSolver {
    board:          Board,
    solutions:      Vec<Vec<u8>>,
    max_depth:      u32,
    node_count:     u64,
    max_node_count: u64,
}

impl IdaSolver {
    pub fn new(max_node_count: u64) -> Self {
        Self {
            board:      Board::new(0, &[0; 64]),
            solutions:  vec![],
            max_depth:  0,
            node_count: 0,
            max_node_count
        }
    }

    fn dfs(&mut self, board: &Board, depth: u32, sol: &Vec<u8>) {
        self.node_count += 1;
        if board.is_solved() {
            eprintln!("{}", sol.iter().join(" "));
            self.solutions.push(sol.clone());
            return;
        }
        if depth + 1 > self.max_depth { return; }
        if board.is_stuck() { return; }
        let moves = board.moves();
        if moves.is_empty() { return; }
        if depth + board.least_to_solve() > self.max_depth { return; }
        if self.node_count > self.max_node_count { return; }

        for to in moves {
            let mut board2 = board.clone();
            board2.move_(to);
            let mut sol2 = sol.clone();
            sol2.push(to);
            self.dfs(&board2, depth+1, &sol2);
        }
    }
}

impl Solver for IdaSolver {
    fn solve(&mut self, board: &Board) -> Result<Vec<Vec<u8>>,SolverError> {
        self.board     = board.clone();
        self.solutions = vec![];
        self.max_depth = 0;

        loop {
            self.node_count = 0;

            eprint!("Depth {}: ", self.max_depth);
            io::stderr().flush().unwrap();
            let t = time::Instant::now();

            let board = self.board.clone();
            self.dfs(&board, 0, &vec![]);

            let mut dur = util::duration_float(&t.elapsed());
            if dur < 1e-3 { dur = 1e-3; }
            eprintln!("Nodes={}, Time={:.3}, NPS={:.0}",
                     self.node_count,
                     dur,
                     self.node_count as f64 / dur);

            if self.node_count > self.max_node_count {
                return Err(SolverError::new(self.solutions.clone()));
            }
            if !self.solutions.is_empty() {
                return Ok(self.solutions.clone());
            }
            self.max_depth += 1;
        }
    }
}
