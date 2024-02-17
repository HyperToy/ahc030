#![allow(non_snake_case)]
#![allow(dead_code)]

use itertools::Itertools;
use proconio::{input, source::line::LineSource};
use rand::Rng;
use std::{
    collections::{HashSet, VecDeque},
    io::{stdin, BufRead, BufReader},
};

const DX: [isize; 4] = [1, 0, -1, 0];
const DY: [isize; 4] = [0, 1, 0, -1];

fn main() {
    let stdin = stdin();
    let mut source = LineSource::new(BufReader::new(stdin.lock()));

    input! {
        from &mut source,
        N: usize, M: usize, _e: f64,
    }
    let mut oil_fields = vec![];
    for _ in 0..M {
        input! {
            from &mut source,
            size: usize,
            points: [(i32, i32); size]
        }
        let points = points
            .into_iter()
            .map(|(i, j)| Point(i, j))
            .collect::<Vec<_>>();
        oil_fields.push(OilField { size, points });
    }

    let oil_count = oil_fields
        .iter()
        .map(|oil_field| oil_field.size)
        .sum::<usize>();

    let seed = 334;
    let mut rng: rand::rngs::StdRng = rand::SeedableRng::seed_from_u64(seed);

    let mut count = 0;
    let mut has_oil = HashSet::new();
    let mut seen = vec![vec![false; N]; N];
    let mut queue = VecDeque::new();

    let mut remain = N * N * 2;
    'outer: loop {
        let i = rng.gen_range(0..N);
        let j = rng.gen_range(0..N);
        // eprintln!("({}, {})", i, j);
        if seen[i][j] {
            continue;
        }
        seen[i][j] = true;

        if remain == 1 {
            break;
        }
        let res = query1(Point(i as i32, j as i32), &mut source);
        remain -= 1;
        // eprintln!("outer: {}", res);
        if res == 0 {
            continue;
        }
        count += res;
        has_oil.insert((i, j));
        if count == oil_count {
            break 'outer;
        }

        queue.push_back((i, j));
        while !queue.is_empty() {
            let &(i, j) = queue.front().unwrap();
            // eprintln!("({}, {})", i, j);
            queue.pop_front();
            for k in 0..4 {
                let ni = i as isize + DX[k];
                let nj = j as isize + DY[k];
                if ni < 0 || N as isize <= ni || nj < 0 || N as isize <= nj {
                    continue;
                }
                let ni = ni as usize;
                let nj = nj as usize;
                if seen[ni][nj] {
                    continue;
                }
                seen[ni][nj] = true;
                if remain == 1 {
                    break 'outer;
                }
                let res = query1(Point(ni as i32, nj as i32), &mut source);
                remain -= 1;
                // eprintln!("inner: {}", res);
                if res == 0 {
                    continue;
                }
                count += res;
                has_oil.insert((ni, nj));
                if count == oil_count {
                    break 'outer;
                }
                queue.push_back((ni, nj));
            }
        }
    }
    let res = answer(has_oil, &mut source);
    assert_eq!(res, 1);
}

fn query1<R: BufRead>(p: Point, source: &mut LineSource<R>) -> usize {
    println!("q 1 {} {}", p.0, p.1);
    input! {
        from source,
        res: usize,
    }
    res
}
fn query2<R: BufRead>(points: &Vec<Point>, source: &mut LineSource<R>) -> usize {
    println!(
        "q {} {}",
        points.len(),
        points.iter().map(|p| format!("{} {}", p.0, p.1)).join(" ")
    );
    input! {
        from source,
        res: usize,
    }
    res
}
fn answer<R: BufRead>(points: HashSet<(usize, usize)>, source: &mut LineSource<R>) -> usize {
    println!(
        "a {} {}",
        points.len(),
        points
            .into_iter()
            .map(|p| format!("{} {}", p.0, p.1))
            .join(" ")
    );
    input! {
        from source,
        res: usize,
    }
    res
}
struct OilField {
    size: usize,
    points: Vec<Point>,
}
struct Point(i32, i32);
