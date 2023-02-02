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
        _n: usize,
        s: String
    };
    let mut stack = Vec::new();
    for c in s.chars() {
        stack.push(c);
        while stack.len() > 2 {
            let m = stack.len();
            if stack[m - 3] == 'f' && stack[m - 2] == 'o' && stack[m - 1] == 'x' {
                stack.pop();
                stack.pop();
                stack.pop();
            } else {
                break;
            }
        }
    }
    println!("{}", stack.len());
}
