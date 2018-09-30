#[macro_use] extern crate failure;
extern crate generic_array;
extern crate itertools;
extern crate rand;

pub mod board;
pub mod util;

pub mod bfs;
pub mod genetic;
pub mod ida;
pub mod iddfs;

pub trait Solver {
    fn solve(&mut self, board: &board::Board) -> Result<Vec<Vec<u8>>,SolverError>;
}

#[derive(Fail, Debug)]
#[fail(display = "SolverError: solutions: {:?}", sols)]
pub struct SolverError {
    sols: Vec<Vec<u8>>,
}

impl SolverError {
    fn new(sols: Vec<Vec<u8>>) -> Self {
        Self {
            sols,
        }
    }
    #[allow(dead_code)]
    pub fn solutions(&self) -> &[Vec<u8>] {
        &self.sols
    }
}
