#[macro_use] extern crate failure;
extern crate permutohedron;

use std::char;
use std::fmt;
use std::io::{ self, prelude::* };
use std::iter::{ FromIterator };
use std::str::{ FromStr };

use permutohedron::{ LexicalPermutation };

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn distance(&self, other: &Point) -> i32 {
        (self.x-other.x).abs() + (self.y-other.y).abs()
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

#[test]
fn test_point() {
    let p1 = Point::new(0, 0);
    let p2 = Point::new(3, 7);
    let p3 = Point::new(8, 3);

    assert_eq!(0, p1.distance(&p1));
    assert_eq!(0, p2.distance(&p2));
    assert_eq!(0, p3.distance(&p3));

    assert_eq!(10, p1.distance(&p2));
    assert_eq!(10, p2.distance(&p1));
    assert_eq!( 9, p2.distance(&p3));
    assert_eq!( 9, p3.distance(&p2));
    assert_eq!(11, p3.distance(&p1));
    assert_eq!(11, p1.distance(&p3));
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Problem {
    start: Point,
    rooms: Vec<Point>,
}

impl fmt::Display for Problem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut buf = vec![vec!['.'; 7]; 5];

        {
            let p = &mut buf[self.start.y as usize][self.start.x as usize];
            assert_eq!(*p, '.');
            *p = 's';
        }

        for (i,room) in self.rooms.iter().enumerate() {
            let p = &mut buf[room.y as usize][room.x as usize];
            assert_eq!(*p, '.');
            *p = char::from_digit(i as u32, 10).unwrap();
        }

        for row in buf {
            writeln!(f, "{}", String::from_iter(row))?;
        }
        Ok(())
    }
}

impl FromStr for Problem {
    type Err = failure::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let buf: Vec<_> = s.lines().map(|l| Vec::from_iter(l.chars())).collect();

        let mut start = Point::new(-1,-1);
        let mut rooms = vec![Point::new(-1,-1); 6];

        for y in 0..5_i32 {
            for x in 0..7_i32 {
                let c = buf[y as usize][x as usize];
                match c {
                    's' => {
                        start = Point::new(x, y);
                    },
                    '0'...'5' => {
                        rooms[c.to_digit(10).unwrap() as usize] = Point::new(x, y);
                    },
                    '.' => {},
                    _ => bail!("unexpected char: {}", c),
                }
            }
        }

        assert!(start.x >= 0 && start.y >= 0);
        assert!(rooms.iter().all(|p| p.x >= 0 && p.y >= 0));

        Ok(Self { start, rooms })
    }
}

#[test]
fn test_problem() {
    let problem_str = "\
2....0.
..s....
3......
..14...
......5
";
    let problem: Problem = problem_str.parse().unwrap();
    assert_eq!(problem_str, format!("{}", problem));
}

fn calc_step(problem: &Problem, perm: &[usize]) -> i32 {
    let mut step = 0;

    let first = &problem.rooms[*perm.first().unwrap()];
    let last  = &problem.rooms[*perm.last().unwrap()];
    step += problem.start.distance(first);
    step += problem.start.distance(last);

    let mut pairs = perm.windows(2);
    while let Some([i,j]) = pairs.next() {
        let src = problem.rooms[*i];
        let dst = problem.rooms[*j];
        step += src.distance(&dst);
    }

    step
}

fn solve(problem: &Problem) -> (Vec<Vec<usize>>, i32) {
    let mut res = vec![];
    let mut step_min = 999999999;

    let mut perm = [0, 1, 2, 3, 4, 5];
    loop {
        let step = calc_step(problem, &perm);
        if step <= step_min {
            if step < step_min { res.clear(); }
            res.push(perm.to_vec());
            step_min = step;
        }
        if !perm.next_permutation() { break; }
    }

    (res, step_min)
}

fn main() -> Result<(), failure::Error> {
    let mut s = String::new();
    io::stdin().read_to_string(&mut s)?;
    let problem = Problem::from_str(&s)?;

    let (sols, step) = solve(&problem);

    println!("step={}", step);
    for sol in sols {
        println!("{:?}", sol);
    }

    Ok(())
}
