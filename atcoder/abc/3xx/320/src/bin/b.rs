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
use itertools::{chain, iproduct, iterate, izip, Itertools};
#[allow(unused_imports)]
use itertools_num::ItertoolsNum;
#[allow(unused_imports)]
use proconio::marker::Chars;
#[allow(unused_imports)]
use proconio::marker::Usize1;
#[allow(unused_imports)]
use proconio::{input, source::line::LineSource};
#[allow(unused_imports)]
use rustc_hash::FxHashMap;
#[allow(unused_imports)]
use rustc_hash::FxHashSet;
#[allow(unused_imports)]
use std::io::BufRead;

fn is_kaibun(s: &[char]) -> bool {
    let n = s.len();
    (0..(n / 2)).all(|i| s[i] == s[n - 1 - i])
}

fn main() {
    input! {
        s: Chars
    };
    let n = s.len();
    let res = (0..n)
        .map(|i| (i..n).filter(|&j| is_kaibun(&s[i..j])).map(|j| j - i + 1))
        .max()
        .unwrap();
    println!("{}", res);
}
