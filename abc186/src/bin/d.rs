use proconio::input;
use proconio::source::auto::AutoSource;

fn resolve(n: usize, a: Vec<i64>) -> i64 {
    let mut a = a;
    a.sort();
    let mut plus_total: i64 = 0;
    let mut res: i64 = 0;
    for i in 0..n {
        if i + 1 == n {
            break;
        }
        plus_total += a[i];
        res += (a[i + 1] * (i + 1) as i64) - plus_total;
    }
    res
}

#[cfg(test)]
mod module_test {
    use super::*;

    #[test]
    fn _test_case1() {
        assert_eq!(8, resolve(3, vec![5, 1, 2]))
    }

    #[test]
    fn _test_case2() {
        assert_eq!(176, resolve(5, vec![31, 41, 59, 26, 53]))
    }

    #[test]
    fn _test_case3() {
        assert_eq!(1, resolve(2, vec![1, 2]))
    }
}


fn main() {
    input! {
        n: usize,
        a: [i64; n]
    }
    println!("{}", resolve(n, a))
}
