use proconio::input;

fn resolve(ab_vec: Vec<(i64, i64)>) -> i64 {
    let mut x: i64 = 0;
    let mut calc_ab: Vec<i64> = vec![];

    for ab in ab_vec {
        let (a, b) = ab;
        x -= a;
        calc_ab.push((a + a + b) as i64);
    }

    calc_ab.sort();

    let mut res: i64 = 0;

    while x <= 0 {
        x += calc_ab.pop().unwrap();
        res += 1;
    }

    res
}

#[cfg(test)]
mod module_test {
    use super::*;

    #[test]
    fn _test_case1() {
        assert_eq!(1, resolve(vec![(2, 1), (2, 2), (5, 1), (1, 3)]))
    }

    #[test]
    fn _test_case2() {
        assert_eq!(3, resolve(vec![(2, 1), (2, 1), (2, 1), (2, 1), (2, 1)]))
    }

    #[test]
    fn _test_case3() {
        assert_eq!(1, resolve(vec![(273, 691)]))
    }
}


fn main() {
    input! {
        n: usize,
        ab: [(i64, i64); n]
    }
    println!("{}", resolve(ab))
}
