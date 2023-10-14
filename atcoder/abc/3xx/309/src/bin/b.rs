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
        a: [Chars; n]
    };
    for i in 0..n {
        let res = (0..n)
            .map(|j| {
                if i == 0 {
                    if j == 0 {
                        a[1][0]
                    } else {
                        a[0][j - 1]
                    }
                } else if i == n - 1 {
                    if j == n - 1 {
                        a[n - 2][n - 1]
                    } else {
                        a[n - 1][j + 1]
                    }
                } else {
                    if j == 0 {
                        a[i + 1][0]
                    } else if j == n - 1 {
                        a[i - 1][n - 1]
                    } else {
                        a[i][j]
                    }
                }
            })
            .collect_vec();
        println!("{}", res.into_iter().join(""));
    }
}
