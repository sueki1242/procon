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

const LIM: i64 = 10_000_000_000;

fn solve(n: usize, t: Vec<char>) -> i64 {
    if n == 1 {
        return if t[0] == '0' { LIM } else { LIM * 2 };
    }
    let mut ss = Vec::new();
    for _ in 0..n {
        ss.push('1');
        ss.push('1');
        ss.push('0');
    }
    let start = match (0..3).filter(|&i| &t[0..n] == &ss[i..(n + i)]).nth(0) {
        None => return 0,
        Some(x) => x as i64,
    };
    let unit = (start + n as i64 + 2) / 3;
    LIM - unit + 1
}

fn main() {
    input! {
        n: usize,
        t: Chars
    };
    let res = solve(n, t);
    println!("{}", res);
}
