use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u64; n]
    };
    let res = a.into_iter().fold(1u64, |sum, x| sum.saturating_mul(x));
    if res <= 10u64.pow(18) {
        println!("{}", res)
    } else {
        println!("-1");
    };
}
