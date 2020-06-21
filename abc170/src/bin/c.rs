use proconio::input;
use std::cmp;

fn main() {
    input! {
        x: i128,
        n: i128,
        mut p: [i128; n]
    };
    if n == 0 {
        println!("{}", x);
        return;
    }
    let mut y = x;
    loop {
        if p.iter().find(|&&x| y == x) == None {
            break;
        }
        y = y + 1;
    }

    let mut z = x - 1;

    loop {
        if p.iter().find(|&&x| z == x) == None {
            break;
        }
        z = z - 1;
    }

    let ans = if (x - y).abs() < (x - z).abs() {
        if y == z {
            cmp::min(y, z)
        } else {
            y
        }
    } else {
        if y == z {
            cmp::min(y, z)
        } else {
            z
        }
    };
    println!("{}", ans);
}
