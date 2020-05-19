use proconio::input;
// use num::*;

fn main() {
    input! {
        k: usize,
        s: String
    };
    let res = if s.len() <= k {
        s
    } else {
        format!("{}{}", &s[0..k], "...")
    };
    println!("{}", res);
}
