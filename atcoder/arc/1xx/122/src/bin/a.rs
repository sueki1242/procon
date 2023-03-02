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

const MOD: i64 = 1_000_000_007;

fn main() {
    input! {
        n: usize,
        a: [i64; n]
    };
    let fib = (0..(n + 1))
        .scan((0, 0, 1), |cum, _| {
            *cum = (cum.1, cum.2, (cum.1 + cum.2) % MOD);
            Some(*cum)
        })
        .map(|v| v.1)
        .collect_vec();
    let total = fib[n];
    let res = (0..n)
        .map(|i| {
            let pos = fib[i] * fib[n - i] % MOD;
            let neg = (total - pos + MOD) % MOD;
            (pos - neg + MOD) * a[i] % MOD
        })
        .sum::<i64>()
        % MOD;
    println!("{}", res);
}
