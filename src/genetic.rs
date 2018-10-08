/**
 * 遺伝的アルゴリズムらしきもの
 *
 * 個体は u8 の配列で表現する。
 * 実際に Board へ適用するときは Board::moves() のサイズで剰余をとる。
 */

use std::cmp;
//use std::io::{ self, prelude::* };
//use std::time;

//use itertools::{ Itertools };
use rand::{ prelude::*, distributions::{ Weighted, WeightedChoice } };

use ::{ Solver, SolverError };
use board::{ Board };
//use util;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum BoardState {
    Solved,
    Stuck,
    NoMove,
    Playing,
}

#[derive(Debug)]
struct Candidate {
    v:      Vec<u8>,
    state:  BoardState,
    rotate: u32,
    step:   u32,
    score:  u32,
}

impl Candidate {
    fn new(board: &Board, v: Vec<u8>, k_ini: u32) -> Self {
        let mut board  = board.clone();
        let mut state  = BoardState::Playing;
        let mut rotate = 0;
        let mut step   = 0;

        // 初期局面はまだ解かれておらず、手詰まりでもないと仮定している
        for &e in &v {
            let moves = board.moves();
            let idx = e as usize % moves.len();
            let dst = moves[idx];
            rotate += 1;
            step   += board.calc_step(board.pos, dst).unwrap();
            board.move_(dst);

            if board.is_solved() {
                state = BoardState::Solved;
                break;
            }
            if board.is_stuck() {
                state = BoardState::Stuck;
                break;
            }
            let moves = board.moves();
            if moves.is_empty() {
                state = BoardState::NoMove;
                break;
            }
        }

        let score = Candidate::evaluate(&board, state, rotate, step, k_ini);

        Self {
            v,
            state,
            rotate,
            step,
            score,
        }
    }

    fn evaluate(board: &Board, state: BoardState, rotate: u32, step: u32, k_ini: u32) -> u32 {
        if state == BoardState::Solved {
            return 100000 - 100*rotate - step;
        }

        /*
        let cnt = board.counts();
        let pena1: u32 = cnt.iter()
            .fold(0, |sum,&e| sum + u32::from(e));
        let pena2 = cnt.iter()
            .filter(|&e| e % 2 != 0)
            .count() as u32;
        let pena = 10*pena1 + 3*pena2;
        */
        let cnt = board.counts();
        let k: u32 = cnt.iter()
            .fold(0, |sum,&e| sum + u32::from(e));

        match state {
            BoardState::Solved => unreachable!(),
            BoardState::Stuck => {
                1000 + 150*rotate - 800*k.pow(2) / k_ini.pow(2)
            },
            BoardState::NoMove => {
                1000 + 150*rotate - 800*k.pow(2) / k_ini.pow(2)
            },
            BoardState::Playing => {
                10000 + 150*rotate - 8000*k.pow(2) / k_ini.pow(2)
            },
        }
    }

    fn extract(&self, len: u32) -> Vec<u8> {
        self.v[..len as usize].to_vec()
    }

    fn to_solution(&self, board: &Board) -> Vec<u8> {
        let mut res = vec![];

        let mut board = board.clone();
        for i in 0..self.rotate {
            let e = self.v[i as usize];
            let moves = board.moves();
            let idx = e as usize % moves.len();
            let pos = moves[idx];
            res.push(pos);
            board.move_(pos);
        }

        res
    }
}

pub struct GeneticSolver {
    board:   Board,
    max_len: u32,
    n_gene:  u32,
    cands:   Vec<Candidate>,
    rng:     ThreadRng,
    k_ini:   u32,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Operation {
    COPY,
    CROSSOVER,
    MUTATE,
}

impl GeneticSolver {
    const N_CAND:  usize = 1000;
    const N_ELITE: usize = 10;

    pub fn new(max_len: u32, n_gene: u32) -> Self {
        Self {
            board: Board::new(0, &[0; 64]),
            max_len,
            n_gene,
            cands: vec![],
            rng:   thread_rng(),
            k_ini: 0,
        }
    }

    fn evolve(&mut self) {
        let mut vs = Vec::with_capacity(GeneticSolver::N_CAND);

        for i in 0..GeneticSolver::N_ELITE {
            vs.push(self.cands[i].extract(self.max_len));
        }

        let mut ops = vec![
            Weighted { item: Operation::COPY,      weight:  9 },
            Weighted { item: Operation::CROSSOVER, weight: 90 },
            Weighted { item: Operation::MUTATE,    weight:  1 },
        ];
        let ops_wc = WeightedChoice::new(&mut ops);

        let mut select = vec![];
        for (i,cand) in self.cands.iter().enumerate() {
            select.push(Weighted { item: i, weight: cand.score });
        }
        let select_wc = WeightedChoice::new(&mut select);

        let n_iter = GeneticSolver::N_CAND - GeneticSolver::N_ELITE;
        for _ in 0..n_iter {
            let op = ops_wc.sample(&mut self.rng);
            let v = match op {
                Operation::COPY      => self.spawn_copy(&select_wc),
                Operation::CROSSOVER => self.spawn_crossover(&select_wc),
                Operation::MUTATE    => self.spawn_mutate(&select_wc),
            };
            vs.push(v);
        }

        self.update_cands(vs);
    }

    fn spawn_copy(&mut self, select_wc: &WeightedChoice<usize>) -> Vec<u8> {
        let i = select_wc.sample(&mut self.rng);
        self.cands[i].extract(self.max_len)
    }

    fn spawn_crossover(&mut self, select_wc: &WeightedChoice<usize>) -> Vec<u8> {
        let i1 = select_wc.sample(&mut self.rng);
        let i2 = select_wc.sample(&mut self.rng);
        let v1 = self.cands[i1].extract(self.max_len);
        let v2 = self.cands[i2].extract(self.max_len);
        debug_assert_eq!(v1.len(), v2.len());

        self.crossover_twopoint(&v1, &v2)
    }

    fn crossover_twopoint(&mut self, v1: &[u8], v2: &[u8]) -> Vec<u8> {
        let start = self.rng.gen_range(0, v1.len());
        let end   = self.rng.gen_range(start+1, v1.len()+1);

        let mut res = Vec::with_capacity(v1.len());
        res.extend_from_slice(&v1[0..start]);
        res.extend_from_slice(&v2[start..end]);
        if end < v1.len() {
            res.extend_from_slice(&v1[end..]);
        }
        res
    }

    #[allow(dead_code)]
    #[allow(unused_variables)]
    fn crossover_uniform(&mut self, v1: &[u8], v2: &[u8]) -> Vec<u8> {
        // TODO: stub
        vec![]
    }

    fn spawn_mutate(&mut self, select_wc: &WeightedChoice<usize>) -> Vec<u8> {
        let i = select_wc.sample(&mut self.rng);
        let mut v = self.cands[i].extract(self.max_len);

        let j = self.rng.gen_range(0, v.len());
        v[j] = self.rng.gen();
        v
    }

    fn update_cands(&mut self, vs: Vec<Vec<u8>>) {
        //let mut cands = Vec::with_capacity(GeneticSolver::N_CAND);
        self.cands.clear();
        for v in vs {
            let cand = Candidate::new(&self.board, v, self.k_ini);
            if cand.state == BoardState::Solved {
                self.max_len = cmp::min(self.max_len, cand.rotate);
            }
            self.cands.push(cand);
        }
        //self.cands = cands;
        self.cands.sort_unstable_by_key(|cand| cmp::Reverse(cand.score));
    }

    fn random_v(&mut self) -> Vec<u8> {
        (0..self.max_len)
            .map(|_| self.rng.gen())
            .collect()
    }
}

impl Solver for GeneticSolver {
    fn solve(&mut self, board: &Board) -> Result<Vec<Vec<u8>>,SolverError> {
        self.board = board.clone();
        self.k_ini = board.counts().iter()
            .fold(0, |sum,&e| sum + u32::from(e));

        let vs = (0..GeneticSolver::N_CAND)
            .map(|_| self.random_v())
            .collect();
        self.update_cands(vs);

        for i in 0..self.n_gene {
            eprintln!("Generation {}: max_len={}", i, self.max_len);

            self.evolve();
            if cfg!(debug_assertions) {
                eprintln!("{:?}", &self.cands[..20]);
            }
        }

        let mut res: Vec<_> = self.cands.iter()
            .filter_map(|cand| {
                match cand.state {
                    BoardState::Solved => Some(cand.to_solution(&self.board)),
                    _                  => None,
                }
            })
            .collect();
        res.sort();
        res.dedup();
        Ok(res)
    }
}
