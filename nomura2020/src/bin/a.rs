use chrono::prelude::*;
use chrono::Duration;
use proconio::input;
// use num::*;

fn main() {
    input! {
        h1: u32,
        m1: u32,
        h2: u32,
        m2: u32,
        k: i64
    };
    let dt = Utc.ymd(2020, 5, 30).and_hms(h1, m1, 0);
    let dt2 = Utc.ymd(2020, 5, 30).and_hms(h2, m2, 0);
    let dt2 = dt2 - Duration::minutes(k);
    let ans = dt2 - dt;
    println!("{}", ans.num_minutes());
}
