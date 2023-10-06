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

fn main() {
    input! {
        s: Chars
    };
    let res = s
        .windows(2)
        .sorted()
        .group_by(|x| *x)
        .into_iter()
        .map(|(key, group)| (key.iter().join(""), group.count()))
        .max_by_key(|(key, count)| (*count, Reverse(key.clone())))
        .unwrap()
        .0;
    println!("{}", res);
}
