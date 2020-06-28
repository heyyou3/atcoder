use proconio::input;
// use num::*;

fn main() {
    input! {
      n: i128,
      k: usize,
      mut p: [usize; n]
    };
    p.sort();
    let mut index: usize = 0;
    let mut ans = 0;
    while index < k {
        ans += p[index];
        index = index + 1;
    }
    println!("{}", ans);
}
