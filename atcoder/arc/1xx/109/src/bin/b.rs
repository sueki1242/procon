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

fn solve(n: i64) -> i64 {
    if n <= 2 {
        return 1;
    }
    // (n+1) のやつで 1..x のxがどこまで含まれるかをにぶたん
    let mut l = 1;
    let mut r = 2_000_000_001;
    while l + 1 < r {
        let med = (l + r) / 2;
        let val = (med + 1) * med / 2;
        if val <= n + 1 {
            l = med;
        } else {
            r = med;
        }
    }
    return n - l + 1;
}

fn main() {
    input! {
        n: i64
    };
    println!("{}", solve(n));
}
