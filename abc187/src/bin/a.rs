use proconio::input;

fn resolve(a: Vec<usize>, b: Vec<usize>) -> usize {
    std::cmp::max(a.into_iter().sum(), b.into_iter().sum())
}

#[cfg(test)]
mod module_test {
    use super::*;

    #[test]
    fn _test_case1() {
        assert_eq!(9, resolve(vec![1, 2, 3], vec![2, 3, 4]))
    }

    #[test]
    fn _test_case2() {
        assert_eq!(17, resolve(vec![5, 9, 3], vec![9, 5, 3]))
    }

    #[test]
    fn _test_case3() {
        assert_eq!(27, resolve(vec![1, 0, 0], vec![9, 9, 9]))
    }
}


fn main() {
    input! {
        a: (String, String)
    }
    let A: Vec<usize> = a.0.chars().map(|c| c.to_string().parse().unwrap()).collect();
    let B: Vec<usize> = a.1.chars().map(|c| c.to_string().parse().unwrap()).collect();
    println!("{}", resolve(A, B));
}
