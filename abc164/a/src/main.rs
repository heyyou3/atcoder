use proconio::input;

fn main() {
    input! {
        s: i64,
        w: i64
    };
    let ans = if s <= w {
        "unsafe"
    } else {
        "safe"
    };
    println!("{}", ans);
}
