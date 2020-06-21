use proconio::input;
// use num::*;

fn main() {
    input! {
        x: [i128; 5],
    };
    let mut ans = 0;
    for (i, &val) in x.iter().enumerate() {
        if val == 0 {
            ans = i + 1
        }
    }
    println!("{}", ans);
}
