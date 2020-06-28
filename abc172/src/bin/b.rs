use proconio::input;
// use num::*;

fn main() {
    input! {
        s: String,
        t: String,
    };
    let mut count = 0;
    for (i, c) in s.as_str().chars().enumerate() {
        if c != t.chars().collect::<Vec<char>>()[i] {
            count = count + 1;
        }
    }
    println!("{}", count);
}
