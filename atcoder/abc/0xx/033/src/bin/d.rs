#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
#[allow(unused_imports)]
use std::io;
#[allow(unused_imports)]
use std::iter::*;
#[allow(unused_imports)]
use std::mem::*;
#[allow(unused_imports)]
use std::str::*;
#[allow(unused_imports)]
use std::usize;

#[allow(unused_imports)]
use bitset_fixed::BitSet;
#[allow(unused_imports)]
use itertools::{chain, iproduct, iterate, izip, zip, Itertools};
#[allow(unused_imports)]
use itertools_num::ItertoolsNum;
#[allow(unused_imports)]
use proconio::marker::{Chars, Usize1};
#[allow(unused_imports)]
use proconio::{input, source::line::LineSource};
#[allow(unused_imports)]
use rustc_hash::FxHashMap;
#[allow(unused_imports)]
use rustc_hash::FxHashSet;
#[allow(unused_imports)]
use std::io::BufRead;

use std::f64::consts::*;

const EPS: f64 = 1e-12;

fn zero(x: f64) -> bool {
    x.abs() <= EPS
}

fn solve(xy: &Vec<(i32, i32)>, i: usize) -> (usize, usize) {
    let henkakus = xy
        .iter()
        .enumerate()
        .filter(|&(ii, _)| ii != i)
        .map(|(_, &v)| ((v.0 - xy[i].0) as f64).atan2((v.1 - xy[i].1) as f64))
        .sorted_by(|a, b| a.partial_cmp(b).unwrap())
        .collect_vec();
    let henkakus2 = henkakus
        .iter()
        .map(|&v| v)
        .chain(henkakus.iter().map(|v| v + 2.0 * PI))
        .collect_vec();
    let mut over90 = 0;
    let mut over180 = 0;
    let mut res = (0, 0);
    for i in 0..(xy.len() - 1) {
        while henkakus2[over90] - henkakus2[i] + EPS < FRAC_PI_2 {
            over90 += 1;
        }
        while henkakus2[over180] - henkakus2[i] + EPS < PI {
            over180 += 1;
        }
        if zero(henkakus2[over90] - henkakus2[i] - FRAC_PI_2) {
            res.0 += 1;
            res.1 += over180 - over90 - 1;
        } else {
            res.1 += over180 - over90;
        }
    }
    res
}

fn main() {
    input! {
        n: usize,
        xy: [(i32, i32); n]
    };
    let mut res = (0, 0);

    for i in 0..n {
        let tmp = solve(&xy, i);
        res.0 += tmp.0;
        res.1 += tmp.1;
    }

    let total = n * (n - 1) * (n - 2) / 6;
    println!("{} {} {}", total - res.0 - res.1, res.0, res.1);
}
