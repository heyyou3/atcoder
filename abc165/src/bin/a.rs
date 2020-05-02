use proconio::input;

fn main() {
    input! {
        k: i128,
        a: i128,
        b: i128,
    };
    let res = (a..b+1).find(|x| {
        x%k == 0
    });
    let ans = if res != None {
        "OK"
    } else {
        "NG"
    };
    println!("{}", ans);
}

