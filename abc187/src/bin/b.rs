use proconio::input;
use proconio::source::auto::AutoSource;

fn resolve(n: usize, xy_vec: Vec<(i64, i64)>) -> usize {
    let mut res = 0;
    if n <= 1 {
        return res;
    }
    for i in 0..xy_vec.len() {
        if (i + 1) == n {
            break;
        }
        let (x1, y1) = xy_vec[i];
        for j in i + 1..xy_vec.len() {
            let (x2, y2) = xy_vec[j];
            let x = x2 as f64 - x1 as f64;
            let y = y2 as f64 - y1 as f64;
            let slope = (y / x);
            if slope >= -1f64 && slope <= 1f64 {
                res += 1
            }
        }
    }
    res
}

#[cfg(test)]
mod module_test {
    use super::*;

    #[test]
    fn _test_case1() {
        assert_eq!(2, resolve(3, vec![(0, 0), (1, 2), (2, 1)]))
    }

    #[test]
    fn _test_case2() {
        assert_eq!(0, resolve(1, vec![(-691, 273)]))
    }

    #[test]
    fn _test_case3() {
        assert_eq!(11, resolve(10, vec![(-31, -35), (8, -36), (22, 64), (5, 73), (-14, 8), (18, -58), (-41, -85), (1, -88), (-21, -85), (-11, 82)]))
    }
}


fn main() {
    input! {
        n: usize,
        xy: [(i64, i64); n]
    }
    println!("{}", resolve(n, xy))
}
