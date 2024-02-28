#![allow(non_snake_case)]
#![allow(dead_code)]

use itertools::Itertools;
use proconio::{input, source::line::LineSource};
use rand::Rng;
use std::{
    collections::{HashSet, VecDeque},
    f64::consts::PI,
    io::{stdin, BufRead, BufReader},
};

const DX: [isize; 4] = [1, 0, -1, 0];
const DY: [isize; 4] = [0, 1, 0, -1];

type Board = Vec<Vec<i32>>;

fn main() {
    let stdin = stdin();
    let mut source = LineSource::new(BufReader::new(stdin.lock()));

    input! {
        from &mut source,
        N: usize, M: usize, e: f64,
    }
    let mut oil_fields = vec![];
    for _ in 0..M {
        input! {
            from &mut source,
            size: usize,
            points: [(isize, isize); size]
        }
        let points = points
            .into_iter()
            .map(|(i, j)| Point(i, j))
            .collect::<Vec<_>>();
        oil_fields.push(OilField { size, points });
    }
    let seed = 334; // なんでや！
    let mut rng: rand::rngs::StdRng = rand::SeedableRng::seed_from_u64(seed);

    if M == 2 {
        let X: Vec<Board> = make_all_candidates(N, M, &oil_fields);
        let n = X.len();

        // 初期確率はすべて等しい
        let mut probabilities = vec![1.0 / n as f64; n];

        let mut loop_count = 0;
        'outer: loop {
            if loop_count > N * N * 2 {
                break;
            }
            loop_count += 1;

            let v = if loop_count == 1 {
                let mut st = HashSet::new();
                for i in 0..N {
                    for j in 0..N / 2 {
                        st.insert(Point(i as isize, j as isize));
                    }
                }
                st
            } else if loop_count == 2 {
                let mut st = HashSet::new();
                for i in 0..N / 2 {
                    for j in 0..N {
                        st.insert(Point(i as isize, j as isize));
                    }
                }
                st
            } else {
                // random にセルを選ぶ
                let k = N * N / 3;
                let mut st = HashSet::new();
                while st.len() < k {
                    let i = rng.gen_range(0..N) as isize;
                    let j = rng.gen_range(0..N) as isize;
                    st.insert(Point(i, j));
                }
                st
            }
            .into_iter()
            .collect::<Vec<_>>();
            // 点集合について query を投げる
            let r = query2(&v, &mut source);

            for i in 0..n {
                // i番目の盤面を仮定した時の v(S) の値を取得
                let mut s = 0;
                for point in v.iter() {
                    s += X[i][point.0 as usize][point.1 as usize];
                }
                // 事前確率に尤度を掛ける
                probabilities[i] *= likelihood(v.len(), s, r, e);
            }
            probabilities = normalize(probabilities);

            if probabilities.iter().any(|p| *p > 0.8) {
                let mut max_index = 0; // 各可能盤面の確率のうち、最も自信度が高いもの の index
                for i in 0..n {
                    if probabilities[i] > probabilities[max_index] {
                        max_index = i;
                    }
                }
                let mut v = HashSet::new();
                let x: Board = X[max_index].clone();
                for i in 0..N {
                    for j in 0..N {
                        if x[i][j] > 0 {
                            v.insert((i, j));
                        }
                    }
                }
                let res = answer(v, &mut source);
                if res == 1 {
                    break 'outer;
                } else {
                    probabilities[max_index] = 0.;
                }
            }
        }
        // }
    } else {
        let oil_count = oil_fields
            .iter()
            .map(|oil_field| oil_field.size)
            .sum::<usize>();

        let mut count = 0;
        let mut has_oil = HashSet::new();
        let mut seen = vec![vec![false; N]; N];
        let mut queue = VecDeque::new();

        let mut remain = N * N * 2;
        'outer: loop {
            let i = rng.gen_range(0..N);
            let j = rng.gen_range(0..N);
            if seen[i][j] {
                continue;
            }
            seen[i][j] = true;

            if remain == 1 {
                break;
            }
            let res = query1(Point(i as isize, j as isize), &mut source);
            remain -= 1;
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
                    let res = query1(Point(ni as isize, nj as isize), &mut source);
                    remain -= 1;
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

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
struct Point(isize, isize);

// bfs で可能なすべての盤面を作成
fn make_all_candidates(N: usize, M: usize, oil_fields: &Vec<OilField>) -> Vec<Board> {
    let mut q: Vec<Board> = Vec::new();
    q.push(vec![vec![0; N]; N]);

    for i in 0..M {
        // 各油田
        let mut nq = Vec::new();
        for b in q {
            // 前の周で見た油田までの可能盤面
            let x_max = oil_fields[i].points.iter().map(|p| p.0).max().unwrap() as usize;
            let y_max = oil_fields[i].points.iter().map(|p| p.1).max().unwrap() as usize;
            for j in 0..N - x_max {
                for k in 0..N - y_max {
                    let mut nb = b.clone();
                    for p in oil_fields[i].points.clone() {
                        let x = p.0 as usize;
                        let y = p.1 as usize;
                        nb[j + x][y + k] += 1;
                    }
                    nq.push(nb);
                }
            }
        }
        q = nq;
    }
    q
}

fn likelihood(k: usize, s: i32, r: usize, e: f64) -> f64 {
    let s = s as f64;
    let k = k as f64;
    let mean = (k - s) * e + s * (1. - e);
    let sigma = (k * e * (1. - e)).sqrt();
    let r = r as f64;

    let res = probability_in_range(if r == 0. { -1e10 } else { r - 0.5 }, r + 0.5, mean, sigma);
    res
}

fn probability_in_range(l: f64, r: f64, mean: f64, sigma: f64) -> f64 {
    assert!(l <= r);
    let res = if mean < l {
        probability_in_range(2. * mean - r, 2. * mean - l, mean, sigma)
    } else {
        let p_l = normal_cdf(l, mean, sigma);
        let p_r = normal_cdf(r, mean, sigma);
        p_r - p_l
    };
    res
}

fn normal_cdf(x: f64, mean: f64, sigma: f64) -> f64 {
    let res = 0.5 * (1. + erf((x - mean) / (sigma * (2f64).sqrt())));
    res
}

// 誤差関数
fn erf(x: f64) -> f64 {
    let mut res = 0.;
    let mut m = 1.;
    for n in 0..1000 {
        if n > 0 {
            m *= -1. * x * x / n as f64;
        }
        res += x / (2. * n as f64 + 1.) * m;
    }
    let res = res * 2. / PI.sqrt();
    if res.is_nan() {
        -1.
    } else {
        res
    }
}

fn normalize(v: Vec<f64>) -> Vec<f64> {
    let sum: f64 = v
        .iter()
        .map(|p| {
            if p.is_sign_negative() || p.is_nan() {
                0.
            } else {
                *p
            }
        })
        .sum();

    let res = v
        .into_iter()
        .map(|p| {
            if p.is_sign_negative() || p.is_nan() || (p / sum).is_nan() {
                0.
            } else {
                p / sum
            }
        })
        .collect::<Vec<_>>();
    res
}

#[cfg(test)]
mod tests {
    use crate::erf;
    #[test]
    fn learning_f64() {
        assert_eq!(std::f64::INFINITY, 1. / 0.);
        assert_eq!(std::f64::NEG_INFINITY, -1. / 0.);
        assert!(std::f64::NAN.is_nan());

        const NAN: f64 = std::f64::NAN;
        assert!((0. / 0f64).is_nan());
        assert!((NAN + NAN).is_nan());
        assert!((NAN - NAN).is_nan());
        assert!((NAN * NAN).is_nan());
        assert!((NAN / NAN).is_nan());
    }

    // 誤差関数 erf のチェック
    #[test]
    fn check_erf_positive() {
        assert_eq!(0., erf(0.));
        assert!((erf(0.00) - 0.000000).abs() < 0.000001);
        assert!((erf(0.01) - 0.011283).abs() < 0.000001);
        assert!((erf(0.02) - 0.022565).abs() < 0.000001);
        assert!((erf(0.03) - 0.033841).abs() < 0.000001);
        assert!((erf(0.04) - 0.045111).abs() < 0.000001);
        assert!((erf(0.05) - 0.056372).abs() < 0.000001);
        assert!((erf(0.10) - 0.112463).abs() < 0.000001);
        assert!((erf(0.20) - 0.222703).abs() < 0.000001);
        assert!((erf(0.30) - 0.328627).abs() < 0.000001);
        assert!((erf(0.40) - 0.428392).abs() < 0.000001);
        assert!((erf(0.50) - 0.520500).abs() < 0.000001);
        assert!((erf(0.60) - 0.603856).abs() < 0.000001);
        assert!((erf(0.70) - 0.677801).abs() < 0.000001);
        assert!((erf(0.80) - 0.742101).abs() < 0.000001);
        assert!((erf(0.90) - 0.796908).abs() < 0.000001);
        assert!((erf(1.00) - 0.842701).abs() < 0.000001);
        assert!((erf(1.50) - 0.966105).abs() < 0.000001);
        assert!((erf(2.00) - 0.995322).abs() < 0.000001);
        assert!((erf(3.00) - 0.999978).abs() < 0.000001);
    }
    #[test]
    fn check_erf_negative() {
        assert!((erf(-0.00) - (-0.000000)).abs() < 0.00001);
        assert!((erf(-0.02) - (-0.022565)).abs() < 0.00001);
        assert!((erf(-0.03) - (-0.033841)).abs() < 0.00001);
        assert!((erf(-0.04) - (-0.045111)).abs() < 0.00001);
        assert!((erf(-0.05) - (-0.056372)).abs() < 0.00001);
        assert!((erf(-0.10) - (-0.112463)).abs() < 0.00001);
        assert!((erf(-0.20) - (-0.222703)).abs() < 0.00001);
        assert!((erf(-0.30) - (-0.328627)).abs() < 0.00001);
        assert!((erf(-0.40) - (-0.428392)).abs() < 0.00001);
        assert!((erf(-0.50) - (-0.520500)).abs() < 0.00001);
        assert!((erf(-0.60) - (-0.603856)).abs() < 0.00001);
        assert!((erf(-0.70) - (-0.677801)).abs() < 0.00001);
        assert!((erf(-0.80) - (-0.742101)).abs() < 0.00001);
        assert!((erf(-0.90) - (-0.796908)).abs() < 0.00001);
        assert!((erf(-1.00) - (-0.842701)).abs() < 0.00001);
        assert!((erf(-1.50) - (-0.966105)).abs() < 0.00001);
        assert!((erf(-2.00) - (-0.995322)).abs() < 0.00001);
        assert!((erf(-3.00) - (-0.999978)).abs() < 0.00001);
    }
}
