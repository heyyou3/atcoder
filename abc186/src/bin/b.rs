use proconio::input;
use proconio::source::auto::AutoSource;

fn resolve(h: i32, w: i32, a: Vec<Vec<i32>>) -> i32 {
    let mut v_min = std::i32::MAX;
    let mut a_total = 0;
    for v in a {
        let m = v.iter().min().unwrap();
        if v_min > *m {
            v_min = *m;
        }
        a_total += v.iter().sum::<i32>();
    }
    let res = a_total - (v_min * h * w);
    return if res <= 0 {
        0
    } else {
        res
    };
}

#[cfg(test)]
mod module_test {
    use super::*;

    #[test]
    fn _test_case1() {
        assert_eq!(2, resolve(2, 3, vec![vec![2, 2, 3], vec![3, 2, 2]]))
    }

    #[test]
    fn _test_case2() {
        assert_eq!(792, resolve(3, 3, vec![vec![99, 99, 99], vec![99, 0, 99], vec![99, 99, 99]]))
    }

    #[test]
    fn _test_case3() {
        assert_eq!(0, resolve(3, 2, vec![vec![99, 99, 99], vec![99, 99, 99]]))
    }
}


fn main() {
    input! {
        h: i32,
        w: i32,
        a: [[i32; w]; h]
    }
    println!("{}", resolve(h, w, a))
}
