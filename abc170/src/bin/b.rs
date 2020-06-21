use proconio::input;
// use num::*;

fn main() {
    input! {
      x: i128,
      y: i128,
    };
    let turt = (y - 2 * x) / (4 - 2);
    let ans = if (turt * 4) <= y && (y % 2) == 0 && turt >= 0 {
        "Yes"
    } else {
        "No"
    };
    println!("{}", ans);
}
