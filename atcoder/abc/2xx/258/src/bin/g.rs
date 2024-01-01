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
    let mut bss = vec![BitSet::new(n); n];
    for (i, aa) in a.iter().enumerate() {
        for j in 0..n {
            bss[i].set(j, aa[j] == '1');
        }
    }
    let res = (0..n).combinations(2).map( |v| {
        let (i, j) = (v[0], v[1]);
        if !bss[i][j] {
            0
        } else {
            (&bss[i] & &bss[j]).count_ones() as u64
        }
    }).sum::<u64>();
    println!("{}", res / 3);
}