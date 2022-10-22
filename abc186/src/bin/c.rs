use std::collections::HashSet;
use proconio::input;
use proconio::source::auto::AutoSource;

fn contains_seven(x: usize, d: usize) -> bool {
    let mut x = x;
    while x != 0 {
        if x % d == 7 {
            return true;
        }
        x /= d;
    }
    return false;
}

fn resolve(n: usize) -> usize {
    let mut seven_cnt = 0;
    for i in 1..=n {
        if !contains_seven(i, 8) && !contains_seven(i, 10) {
            seven_cnt += 1;
        }
    }
    return seven_cnt;
}

#[cfg(test)]
mod module_test {
    use super::*;

    #[test]
    fn _test_case1() {
        assert_eq!(17, resolve(20))
    }

    #[test]
    fn _test_case2() {
        assert_eq!(30555, resolve(100000))
    }

    #[test]
    fn _test_case3() {
        assert_eq!(25, resolve(30))
    }

    #[test]
    fn _test_case4() {
        assert_eq!(1, resolve(1))
    }
}


fn main() {
    input! {
        n: usize
    }
    println!("{}", resolve(n))
}
