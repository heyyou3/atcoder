use proconio::input;

fn resolve(n: i64, a: Vec<i64>) -> usize {
    let a_len = 2usize.pow(n as u32);
    let center = a_len / 2;
    let left = a[0..center].into_iter().max().unwrap();
    let right = a[center..a_len].into_iter().max().unwrap();
    let lose = left.min(right);
    return a.iter().copied().position(|x| x == *lose).unwrap() + 1;
}

#[cfg(test)]
mod module_test {
    use super::*;

    #[test]
    fn _test_case1() {
        assert_eq!(2, resolve(2, vec![1, 4, 2, 5]))
    }

    #[test]
    fn _test_case2() {
        assert_eq!(1, resolve(2, vec![3, 1, 5, 4]))
    }

    #[test]
    fn _test_case3() {
        assert_eq!(2, resolve(4, vec![6, 13, 12, 5, 3, 7, 10, 11, 16, 9, 8, 15, 2, 1, 14, 4]))
    }
}


fn main() {
    input! {
        n: i64,
        a: [i64; 2usize.pow(n as u32)]
    }
    println!("{}", resolve(n, a))
}
