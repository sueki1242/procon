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

// 2進数でもたかだか60桁くらい
// 3進数は38桁
// 4進数は30桁（それはそう）
//
// 10進数で18桁
// 100進数で10桁

// 1000進数くらいまでは全部試して、
// あとはたかだか6桁未満だから1024通りくらい試せばOK
// x^5 + x^3 + x^2 + 1 = n みたいな式を解くわけだけど、
// にぶたんで頑張る。オーバーフローに気を付ける。

fn check_single(n: i64, base: i64) -> bool {
    let mut tmp = n;
    while tmp > 0 {
        if tmp % base > 1 {
            return false;
        }
        tmp /= base;
    }
    true
}

const LIM: [i64; 6] = [
    -1,
    1_000_000_000_000_000_000i64,
    1_000_000_000,
    1_000_000,
    31623,
    3982,
];

fn calc(base: i64, iv: &Vec<i64>, msb: usize) -> i64 {
    let mut res = 0;
    let mut current = 1;
    for i in 0..=msb {
        if iv[i] > 0 {
            res += current;
        }
        if 1_100_000_000_000_000_000i64 / base > current {
            current *= base;
        }
    }
    res
}

fn solve_expr(n: i64, iv: Vec<i64>) -> Option<i64> {
    let msb = (0..6).filter(|&i| iv[i] > 0).last().unwrap();
    if msb == 0 {
        // "1" を表しているが n>=2 なので解なし
        return None;
    }
    if msb == 1 {
        // 1次式なので普通に解ける
        return Some(if iv[0] == 1 { n - 1 } else { n });
    }
    let (mut l, mut r) = (0, LIM[msb]);
    while l + 1 < r {
        let med = (l + r) / 2;
        if calc(med, &iv, msb) >= n {
            r = med;
        } else {
            l = med;
        }
    }
    if calc(r, &iv, msb) == n {
        Some(r)
    } else {
        None
    }
}

fn solve(n: i64) -> usize {
    let mut res = (2..=1000)
        .filter(|&i| check_single(n, i))
        .collect::<FxHashSet<_>>();

    if n < 1000 {
        return res.len();
    }

    for i in 1..64 {
        let iv = {
            let mut tmp = Vec::new();
            let mut ii = i;
            for _ in 0..6 {
                tmp.push(ii % 2);
                ii /= 2;
            }
            tmp
        };
        if let Some(x) = solve_expr(n, iv) {
            res.insert(x);
        }
    }

    res.len()
}

fn main() {
    input! {
        t: usize,
        ns: [i64; t]
    };
    for n in ns {
        println!("{}", solve(n));
    }
}
