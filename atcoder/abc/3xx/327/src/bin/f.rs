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


use ac_library::{Monoid, Segtree};

#[derive(Clone)]
struct Cumsum {
    current: i32,
    best: i32
}

impl Monoid for Cumsum {
    type S = Cumsum;
    fn identity() -> Cumsum {
        Cumsum {current: 0, best: 0}
    }
    fn binary_operation(a: &Cumsum, b: &Cumsum) -> Cumsum {
        Cumsum {current: a.current + b.current, best: max(a.best, a.current + b.best)}
    }
}

fn main() {
    input! {
        n: usize,
        d: usize,
        w: usize,
        tx: [(usize, usize); n]
    };
    let mut timeline = vec![Vec::new(); 400200];
    for (t, x) in tx {
        timeline[t].push((x, 1));
        timeline[t].push((x + w, -1));
        timeline[t + d].push((x, -1));
        timeline[t + d].push((x + w, 1));
    }
    let mut segtree = Segtree::<Cumsum>::new(400200);
    let mut res = 0;
    for t in 0..200200 {
        for &(pos, val) in &timeline[t] {
            let mut tmp = segtree.get(pos);
            tmp.current += val;
            tmp.best = tmp.current;
            segtree.set(pos, tmp);
        }
        res = max(res, segtree.all_prod().best);
    }
    println!("{}", res);
}
