use std::collections::{ VecDeque };
use std::time;

use board::{ Board };

#[derive(Debug)]
pub struct Queue<T> {
    v: VecDeque<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Self { v: VecDeque::<T>::new() }
    }

    pub fn with_capacity(n: usize) -> Self {
        Self { v: VecDeque::<T>::with_capacity(n) }
    }

    pub fn push(&mut self, x: T) {
        self.v.push_back(x);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.v.pop_front()
    }

    pub fn clear(&mut self) {
        self.v.clear();
    }

    pub fn is_empty(&self) -> bool {
        self.v.is_empty()
    }

    pub fn len(&self) -> usize {
        self.v.len()
    }
}

pub fn duration_float(dur: &time::Duration) -> f64 {
    dur.as_secs() as f64 + 1e-9 * f64::from(dur.subsec_nanos())
}

pub fn verify_solution(board: &Board, sol: &[u8]) -> bool {
    let mut board = board.clone();
    for &e in sol {
        board.move_(e);
    }
    board.is_solved()
}

pub fn step_of_solution(board: &Board, sol: &[u8]) -> u32 {
    let mut res = 0;
    let mut board = board.clone();
    for &e in sol {
        res += board.calc_step(board.pos, e).unwrap();
        board.move_(e);
    }
    res
}

// 解のリストを (解,STEP数) のリストに変換
// ROTATE数が少ない順にソートする(ROTATE数が同じならSTEP数が少ない順)
pub fn solutions_with_step(board: &Board, sols: &[Vec<u8>]) -> Vec<(Vec<u8>,u32)> {
    let mut res: Vec<(Vec<u8>,u32)> = sols.iter()
        .map(|sol| {
            let step = step_of_solution(board, sol);
            (sol.to_vec(), step)
        })
        .collect();
    res.sort_by_key(|sol| (sol.0.len(), sol.1));
    res
}
