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

pub struct Dsu {
    n: usize,
    // root node: -1 * component size
    // otherwise: parent
    parent_or_size: Vec<i32>,
}

impl Dsu {
    /// Creates a new `Dsu`.
    ///
    /// # Constraints
    ///
    /// - $0 \leq n \leq 10^8$
    ///
    /// # Complexity
    ///
    /// - $O(n)$
    pub fn new(size: usize) -> Self {
        Self {
            n: size,
            parent_or_size: vec![-1; size],
        }
    }

    // `\textsc` does not work in KaTeX
    /// Performs the Uɴɪᴏɴ operation.
    ///
    /// # Constraints
    ///
    /// - $0 \leq a < n$
    /// - $0 \leq b < n$
    ///
    /// # Panics
    ///
    /// Panics if the above constraints are not satisfied.
    ///
    /// # Complexity
    ///
    /// - $O(\alpha(n))$ amortized
    pub fn merge(&mut self, a: usize, b: usize) -> usize {
        assert!(a < self.n);
        assert!(b < self.n);
        let (mut x, mut y) = (self.leader(a), self.leader(b));
        if x == y {
            return x;
        }
        if -self.parent_or_size[x] < -self.parent_or_size[y] {
            std::mem::swap(&mut x, &mut y);
        }
        self.parent_or_size[x] += self.parent_or_size[y];
        self.parent_or_size[y] = x as i32;
        x
    }

    /// Returns whether the vertices $a$ and $b$ are in the same connected component.
    ///
    /// # Constraints
    ///
    /// - $0 \leq a < n$
    /// - $0 \leq b < n$
    ///
    /// # Panics
    ///
    /// Panics if the above constraint is not satisfied.
    ///
    /// # Complexity
    ///
    /// - $O(\alpha(n))$ amortized
    pub fn same(&mut self, a: usize, b: usize) -> bool {
        assert!(a < self.n);
        assert!(b < self.n);
        self.leader(a) == self.leader(b)
    }

    /// Performs the Fɪɴᴅ operation.
    ///
    /// # Constraints
    ///
    /// - $0 \leq a < n$
    ///
    /// # Panics
    ///
    /// Panics if the above constraint is not satisfied.
    ///
    /// # Complexity
    ///
    /// - $O(\alpha(n))$ amortized
    pub fn leader(&mut self, a: usize) -> usize {
        assert!(a < self.n);
        if self.parent_or_size[a] < 0 {
            return a;
        }
        self.parent_or_size[a] = self.leader(self.parent_or_size[a] as usize) as i32;
        self.parent_or_size[a] as usize
    }

    /// Returns the size of the connected component that contains the vertex $a$.
    ///
    /// # Constraints
    ///
    /// - $0 \leq a < n$
    ///
    /// # Panics
    ///
    /// Panics if the above constraint is not satisfied.
    ///
    /// # Complexity
    ///
    /// - $O(\alpha(n))$ amortized
    pub fn size(&mut self, a: usize) -> usize {
        assert!(a < self.n);
        let x = self.leader(a);
        -self.parent_or_size[x] as usize
    }

    /// Divides the graph into connected components.
    ///
    /// The result may not be ordered.
    ///
    /// # Complexity
    ///
    /// - $O(n)$
    pub fn groups(&mut self) -> Vec<Vec<usize>> {
        let mut leader_buf = vec![0; self.n];
        let mut group_size = vec![0; self.n];
        for i in 0..self.n {
            leader_buf[i] = self.leader(i);
            group_size[leader_buf[i]] += 1;
        }
        let mut result = vec![Vec::new(); self.n];
        for i in 0..self.n {
            result[i].reserve(group_size[i]);
        }
        for i in 0..self.n {
            result[leader_buf[i]].push(i);
        }
        result
            .into_iter()
            .filter(|x| !x.is_empty())
            .collect::<Vec<Vec<usize>>>()
    }
}

const MOD: i64 = 998244353;

fn solve(f: Vec<usize>) -> i64 {
    let mut d = Dsu::new(f.len());
    for (i, &v) in f.iter().enumerate() {
        d.merge(i, v);
    }
    let group_num = d.groups().len();
    ((0..group_num).fold(1, |acc, _| acc * 2 % MOD) + (MOD - 1)) % MOD
}

fn main() {
    input! {
        n: usize,
        f: [Usize1; n]
    };
    let res = solve(f);

    println!("{}", res);
}
