use std::io::{ self, prelude::* };
use std::time;

use itertools::{ Itertools };

use ::{ Solver, SolverError };
use board::{ Board };
use util;

pub struct BfsSolver {
    que:            util::Queue<(Board,Vec<u8>)>,
    max_node_count: u64,
}

impl BfsSolver {
    pub fn new(max_node_count: u64) -> Self {
        Self {
            que: util::Queue::with_capacity(max_node_count as usize),
            max_node_count,
        }
    }

    fn search_next(&mut self) -> Result<Vec<Vec<u8>>,SolverError> {
        let mut res = vec![];

        let node_count = self.que.len();
        for _ in 0..node_count {
            let (board, sol) = self.que.pop().unwrap();

            if board.is_solved() {
                eprintln!("{}", sol.iter().join(" "));
                res.push(sol);
                continue;
            }
            if board.is_stuck() { continue; }
            let moves = board.moves();
            if moves.is_empty() { continue; }

            for to in moves {
                let mut board2 = board.clone();
                board2.move_(to);
                let mut sol2 = sol.clone();
                sol2.push(to);
                self.que.push((board2, sol2));
            }

            if self.que.len() > self.max_node_count as usize {
                return Err(SolverError::new(res));
            }
        }

        Ok(res)
    }
}

impl Solver for BfsSolver {
    fn solve(&mut self, board: &Board) -> Result<Vec<Vec<u8>>,SolverError> {
        self.que.clear();
        self.que.push((board.clone(), vec![]));

        let mut depth = 0;
        loop {
            eprint!("Depth {}: ", depth);
            io::stderr().flush().unwrap();
            let t = time::Instant::now();

            let node_count = self.que.len();
            let r = self.search_next();

            let mut dur = util::duration_float(&t.elapsed());
            if dur < 1e-3 { dur = 1e-3; }
            eprintln!("Nodes={}, Time={:.3}, NPS={:.0}",
                     node_count,
                     dur,
                     node_count as f64 / dur);

            match r {
                Ok(sols) => {
                    if !sols.is_empty() {
                        return Ok(sols);
                    }
                },
                Err(e) => { return Err(e); }
            }
            depth += 1;
        }
    }
}
