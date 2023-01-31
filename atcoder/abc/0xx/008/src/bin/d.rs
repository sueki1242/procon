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

fn solve(
    xmin: i64,
    ymin: i64,
    xmax: i64,
    ymax: i64,
    xy: &Vec<(i64, i64)>,
    memo: &mut FxHashMap<(i64, i64, i64, i64), i64>,
) -> i64 {
    let key = (xmin, ymin, xmax, ymax);
    if xmin >= xmax || ymin >= ymax {
        return 0;
    }
    if memo.contains_key(&key) {
        return memo[&key];
    }
    let mut res = 0;
    for &(x, y) in xy {
        if x <= xmin || x >= xmax || y <= ymin || y >= ymax {
            continue;
        }
        res = max(
            res,
            xmax - xmin + ymax - ymin - 3
                + solve(xmin, ymin, x, y, &xy, memo)
                + solve(xmin, y, x, ymax, &xy, memo)
                + solve(x, ymin, xmax, y, &xy, memo)
                + solve(x, y, xmax, ymax, &xy, memo),
        );
    }
    memo.insert(key, res);
    // println!("{:?} -> {}", &key, res);
    res
}

fn main() {
    input! {
        w: i64,
        h: i64,
        n: usize,
        xy: [(i64, i64); n]
    };
    let mut memo = FxHashMap::default();
    let res = solve(0, 0, w + 1, h + 1, &xy, &mut memo);
    println!("{}", res);
}
