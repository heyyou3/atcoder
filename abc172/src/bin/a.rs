use num::*;
use proconio::input;

fn main() {
    input! {
        i: i128,
    };
    println!("{}", i + pow(i, 2) + pow(i, 3));
}
