use proconio::input;
// use num::*;

fn main() {
    input! {
        n: String,
    };
    let ans = match &n[n.len() - 1..] {
        "2" | "4" | "5" | "7" | "9" => "hon",
        "0" | "1" | "6" | "8" => "pon",
        _ => "bon",
    };
    println!("{}", ans);
}
