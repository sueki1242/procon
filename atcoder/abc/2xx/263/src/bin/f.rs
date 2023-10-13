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

fn main() {
    input! {
        n: usize,
        c: [[i64; n]; 1 << n]
    };
    // dp[i][j] : i戦目まで終えてjが勝ち抜いている場合の、jがいる山の勝ち点の合計の最大値
    let mut dp = vec![vec![0; 1 << n]; n + 1];
    for i in 0..n {
        let width = 1 << i;
        let yama_max = (0..((1 << n) / width))
            .map(|j| (0..width).map(|k| dp[i][width * j + k]).max().unwrap())
            .collect_vec();
        for j in 0..(1 << n) {
            let idx = j / width;
            dp[i + 1][j] =
                yama_max[if idx % 2 == 0 { idx + 1 } else { idx - 1 }] + dp[i][j] + c[j][i]
                    - if i > 0 { c[j][i - 1] } else { 0 };
        }
    }
    let res = dp[n].iter().max().unwrap();
    println!("{}", res);
}
