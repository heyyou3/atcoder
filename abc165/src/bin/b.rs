use proconio::input;

fn main() {
    input! {
        x: i64,
    };
    let mut res = 100;
    let mut count = 0;

    while x > res {
        res = res as i64 + (res / 100) as i64;
        count = count + 1;
    }
    println!("{}", count);
}

