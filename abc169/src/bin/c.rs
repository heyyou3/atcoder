use proconio::input;

fn main() {
    input! {
        a: u64,
        b: f64
    };
    let b = (b * 100f64).round() as u64;
    println!("{}", a * b / 100);
}
