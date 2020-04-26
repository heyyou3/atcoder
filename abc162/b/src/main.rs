use proconio::input;

fn main() {
    input! {
        n: i64,
    };
    println!("{}", (1..n+1).filter(|&i| i%3 != 0 && i%5 != 0).fold(0, |sum, a| sum + a));
}
