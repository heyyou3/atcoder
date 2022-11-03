use proconio::input;
use proconio::source::auto::AutoSource;

fn resolve(n: usize, a: Vec<i64>, b: Vec<i64>) -> String {
    let mut total: i64 = 0;
    for i in 0..n {
        total += a[i] * b[i]
    }
    return if total == 0 {
        "Yes".to_string()
    } else {
        "No".to_string()
    };
}

#[cfg(test)]
mod module_test {
    use super::*;

    #[test]
    fn _test_case1() {
        assert_eq!("Yes", resolve(2, vec![-3, 6], vec![4, 2]))
    }

    #[test]
    fn _test_case2() {
        assert_eq!("No", resolve(2, vec![4, 5], vec![-1, -3]))
    }

    #[test]
    fn _test_case3() {
        assert_eq!("Yes", resolve(3, vec![1, 3, 5], vec![3, -6, 3]))
    }
}


fn main() {
    input! {
        n: usize,
        a: [i64; n],
        b: [i64; n]
    }
    println!("{}", resolve(n, a, b))
}
