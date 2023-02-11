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

// calc x^n
fn exp(x: i64, n: i64) -> i64 {
    (0..n).fold(1, |acc, _| acc * x)
}

// calc (b^c) mod 4
fn fastexp_mod4(b: i64, c: i64) -> i64 {
    let b = b % 4;
    if b == 2 && c == 1 {
        2
    } else {
        exp(b, c % 2 + 2) % 4
    }
}

fn solve(a: i64, b: i64, c: i64) -> i64 {
    let bc = fastexp_mod4(b, c) + 4; // 制約上 b^c = 0 となることはない
    let a = a % 10;
    return exp(a, bc) % 10;
}

fn main() {
    input! {
        a: i64,
        b: i64,
        c: i64
    };
    let res = solve(a, b, c);

    println!("{}", res);
}
