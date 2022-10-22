use proconio::input;
use proconio::source::auto::AutoSource;

fn resolve(n: usize, w: usize) -> usize {
    n / w
}

#[cfg(test)]
mod module_test {
    use super::*;

    #[test]
    fn _test_case1() {
        assert_eq!(3, resolve(10, 3))
    }

    #[test]
    fn _test_case2() {
        assert_eq!(1000, resolve(1000, 1))
    }

    #[test]
    fn _test_case3() {
        assert_eq!(2, resolve(5, 2))
    }
}

fn main() {
    input! {
        n: usize,
        w: usize
    }
    println!("{}", resolve(n, w))
}
