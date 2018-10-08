

extern crate egypt;

use std::fs;
use std::path;
use std::str::{ FromStr };

use egypt::board::{ Board };

use egypt::{ Solver };
use egypt::bfs::{ BfsSolver };
use egypt::ida::{ IdaSolver };
use egypt::iddfs::{ IddfsSolver };

#[test]
fn test_bfs_iddfs_ida() {
    const PROBLEMS_EASY: &[&str] = &[
        "00-0.in",
        "00-1.in",
        "00-2.in",
        "00-3.in",
        "00-4.in",
        "00-5.in",
        "01-0.in",
        "01-1.in",
        "01-2.in",
        "01-3.in",
        "01-4.in",
        "01-5.in",
        "02-0.in",
        "02-1.in",
        "02-2.in",
        "02-3.in",
        "02-4.in",
        "02-5.in",
        "03-0.in",
        "03-1.in",
        "03-4.in",
        "04-0.in",
        "04-2.in",
        "04-3.in",
        "04-4.in",
        "05-0.in",
        "05-1.in",
        "05-4.in",
        "05-5.in",
        "06-4.in",
        "06-5.in",
        "08-3.in",
        "09-0.in",
        "09-3.in",
        "10-5.in",
        "12-0.in",
        "13-0.in",
        "13-4.in",
        "15-0.in",
        "17-5.in",
    ];

    const MAX_DEPTH_INI: u32 = 0;

    const MAX_NODE_COUNT_BFS:   u64 =                10_000_000;
    const MAX_NODE_COUNT_IDDFS: u64 = 1_000_000_000_000_000_000;
    const MAX_NODE_COUNT_IDA:   u64 = 1_000_000_000_000_000_000;

    let mut solver_bfs   = BfsSolver::new(MAX_NODE_COUNT_BFS);
    let mut solver_iddfs = IddfsSolver::new(MAX_DEPTH_INI, MAX_NODE_COUNT_IDDFS);
    let mut solver_ida   = IdaSolver::new(MAX_DEPTH_INI, MAX_NODE_COUNT_IDA);

    for filename in PROBLEMS_EASY {
        let path = path::Path::new("problem/").join(filename);
        eprintln!("{}", path.to_str().unwrap());
        let s = fs::read_to_string(path).unwrap();
        let board = Board::from_str(&s).unwrap();

        let mut sols_bfs = solver_bfs.solve(&board).unwrap();
        sols_bfs.sort();
        let mut sols_iddfs = solver_iddfs.solve(&board).unwrap();
        sols_iddfs.sort();
        let mut sols_ida = solver_ida.solve(&board).unwrap();
        sols_ida.sort();

        assert_eq!(sols_bfs,   sols_iddfs);
        assert_eq!(sols_iddfs, sols_ida);
    }
}
