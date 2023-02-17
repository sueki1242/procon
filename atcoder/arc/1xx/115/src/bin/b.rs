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
        c: [[i32; n]; n]
    };
    let b = c.iter().min().unwrap();
    let a = c.iter().map(|v| v[0] - b[0]).collect_vec();
    let valid = (0..n)
        .cartesian_product(0..n)
        .all(|(i, j)| a[i] + b[j] == c[i][j]);
    if valid {
        println!("Yes");
        println!("{}", a.iter().map(|v| v.to_string()).join(" "));
        println!("{}", b.iter().map(|v| v.to_string()).join(" "));
    } else {
        println!("No");
    }
}
