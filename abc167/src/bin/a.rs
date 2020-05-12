use proconio::input;

fn main() {
    input! {
        s: String,
        mut t: String
    };
    t.pop();
    let ans = if s == t { "Yes" } else { "No" };
    println!("{}", ans);
}
