use std::collections::BTreeMap;
use proconio::input;

fn resolve(nc: (usize, i64), abc: Vec<(usize, usize, usize)>) -> i64 {
    let (_, plan_c) = nc;
    let mut p = BTreeMap::<usize, i64>::new();

    for (a, b, c) in abc {
        *p.entry(a).or_default() += c as i64;
        *p.entry(b + 1).or_default() -= c as i64;
    }

    let mut ans: i64 = 0;
    let mut fee: i64 = 0;
    let mut t: usize = 0;

    for (key, v) in p {
        let w = key - t;
        let p_fee = fee;
        fee += v;
        if p_fee > 0 {
            ans += p_fee.min(plan_c as i64) * w as i64;
        }
        t = key;
    }
    return ans;
}

#[cfg(test)]
mod module_test {
    use super::*;

    #[test]
    fn _test_case1() {
        assert_eq!(10, resolve((2, 6), vec![(1, 2, 4), (2, 2, 4)]))
    }

    #[test]
    fn _test_case2() {
        assert_eq!(163089627821228, resolve((5, 1000000000), vec![
            (583563238, 820642330, 44577), (136809000, 653199778, 90962), (54601291, 785892285, 50554), (5797762, 453599267, 65697), (468677897, 916692569, 87409),
        ]))
    }

    #[test]
    fn _test_case3() {
        assert_eq!(88206004785464, resolve((5, 100000), vec![
            (583563238, 820642330, 44577), (136809000, 653199778, 90962), (54601291, 785892285, 50554), (5797762, 453599267, 65697), (468677897, 916692569, 87409),
        ]))
    }
}


fn main() {
    input! {
        nc: (usize, i64),
    }

    let (n, _) = nc;

    input! {
        abc: [(usize, usize, usize); n]
    }
    println!("{}", resolve(nc, abc))
}
