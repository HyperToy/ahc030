#![allow(non_snake_case)]

use itertools::Itertools;
use proconio::{input, source::line::LineSource};
use std::io::{stdin, BufRead, BufReader};

fn main() {
    let stdin = stdin();
    let mut source = LineSource::new(BufReader::new(stdin.lock()));

    input! {
        from &mut source,
        N: i32, M: usize, e: f64,
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

    let mut count = 0;
    let mut has_oil = vec![];
    'outer: for i in 0..N {
        for j in 0..N {
            let res = query1(Point(i, j), &mut source);
            count += res;
            if res != 0 {
                has_oil.push(Point(i, j));
            }
            if count == oil_count {
                break 'outer;
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
fn answer<R: BufRead>(points: Vec<Point>, source: &mut LineSource<R>) -> usize {
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
