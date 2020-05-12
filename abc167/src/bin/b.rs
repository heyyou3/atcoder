use proconio::input;
use std::cmp::*;

fn main() {
    input! {
        a: i64,
        b: i64,
        _c: i64,
        mut k: i64,
    };
    let mut ans = min(a, k);
    if k > a + b {
        ans -= k - a - b;
    }
    println!("{}", ans);
}
