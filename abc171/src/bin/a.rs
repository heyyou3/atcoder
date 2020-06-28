use proconio::input;
// use num::*;

fn main() {
    input! {
        a: char,
    };
    let res = (b'a'..=b'z').find(|&x| x as char == a);
    let ans = if res == None { "A" } else { "a" };
    println!("{}", ans);
}
