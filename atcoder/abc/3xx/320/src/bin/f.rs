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

const INF: i64 = 1 << 60;

fn main() {
    input! {
        n: usize,
        h: usize,
        x: [usize; n],
        pf: [(i64, usize); n - 1]
    };

    // dp[i][j][k] : Xi のところで行きj, 帰りk くらい残っている場合の数
    // ガソスタ使う場合は遷移元側で使う感じで後ろから埋めていく
    let mut dp = vec![vec![vec![INF; h * 2]; h * 2]; n + 1];
    for i in 0..=h {
        // この状態にスタートから必ず遷移できるわけではないが、
        // 不可能な状態だったら後ろから埋めていく過程でdp[0]まで届かなくて結果淘汰されるので気にしなくて良い
        dp[n][i][i] = 0;
    }
    for ii in 0..n {
        let i = n - 1 - ii;
        let diff = if i > 0 { x[i] - x[i - 1] } else { x[0] };
        if diff > h {
            continue;
        }
        let (p, f) = if i < n - 1 { pf[i] } else { (0, 0) };
        for j in 0..=h {
            for k in 0..=h {
                // 地点 x[i] で何もしなかった場合
                if j + diff <= h && k >= diff {
                    dp[i][j + diff][k - diff] = min(dp[i][j + diff][k - diff], dp[i + 1][j][k]);
                }
                // 行きで使った場合
                if k >= diff && j >= diff {
                    dp[i][j][k - diff] =
                        min(dp[i][j][k - diff], dp[i + 1][min(h, j - diff + f)][k] + p);
                }
                // 帰りで使った場合
                if j + diff <= h && min(h, k + f) >= diff {
                    dp[i][j + diff][min(h, k + f) - diff] =
                        min(dp[i][j + diff][min(h, k + f) - diff], dp[i + 1][j][k] + p);
                }
            }
        }
    }
    let mut res = INF;
    for j in 0..=h {
        res = min(res, dp[0][h][j]);
    }
    println!("{}", if res == INF { -1 } else { res });
}
