use proconio::input;
// use num::*;

fn num_alpha(n: i128) -> String {
    let res = if n <= 26 {
        std::char::from_digit((64 + n) as u32, 10)
            .unwrap()
            .to_string()
    } else if n % 26 == 0 {
        format!(
            "{}{}",
            num_alpha(n / 26 - 1),
            std::char::from_digit(90 as u32, 10).unwrap().to_string()
        )
    } else {
        format!(
            "{}{}",
            num_alpha(n / 26),
            std::char::from_digit(64 + (n % 26) as u32, 10)
                .unwrap()
                .to_string()
        )
    };
    res
}

fn main() {
    input! {
        n: i128,
    };
    println!("{}", num_alpha(n));
}
