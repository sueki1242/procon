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

fn create_mat(c: &Vec<u32>) -> Vec<Vec<u32>> {
    let k = c.len();
    let mut res = vec![vec![0; k]; k];
    for i in 0..k {
        res[0][i] = c[i];
        if i > 0 {
            res[i][i - 1] = std::u32::MAX;
        }
    }
    res
}

fn mat_mul(a: &Vec<Vec<u32>>, b: &Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    let k = a.len();
    let mut res = vec![vec![0; k]; k];
    for i in 0..k {
        for j in 0..k {
            for x in 0..k {
                res[i][j] ^= a[i][x] & b[x][j];
            }
        }
    }
    res
}

fn mat_vec_mul(a: &Vec<Vec<u32>>, b: &Vec<u32>) -> Vec<u32> {
    let k = a.len();
    let mut res = vec![0; k];
    for i in 0..k {
        for j in 0..k {
            res[i] ^= a[i][j] & b[j];
        }
    }
    res
}

fn main() {
    input! {
        k: usize,
        m: Usize1,
        a: [u32; k],
        c: [u32; k]
    };

    if m < k {
        println!("{}", a[m]);
        return;
    }

    let mat = create_mat(&c);
    let mut mats = vec![mat; 1];
    let mut steps = vec![1; 1];
    for i in 0..30 {
        steps.push(steps[i] * 2);
        mats.push(mat_mul(&mats[i], &mats[i]));
    }
    let mut rest = m - k + 1;
    let mut state = a.iter().rev().map(|&x| x).collect_vec();

    let mut now = 29;
    while rest > 0 {
        if rest >= steps[now] {
            rest -= steps[now];
            state = mat_vec_mul(&mats[now], &state);
        } else if now > 0 {
            now -= 1;
        }
    }
    println!("{}", state[0]);
}
