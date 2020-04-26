use proconio::input;

fn main() {
    input! {
        a: f64,
        b: f64,
        c: f64,
        d: f64,
    };
    let ans = if (a / d).ceil() >= (c / b).ceil() {
        "Yes"
    } else {
        "No"
    };
    println!("{}", ans);
}
