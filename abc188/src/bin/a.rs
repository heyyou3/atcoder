use proconio::input;

fn resolve(x: usize, y: usize) -> String {
    let yes = "Yes".to_string();
    let no = "No".to_string();
    if (x > y) && (y + 3) > x {
        return yes;
    }
    if (x < y) && (x + 3) > y {
        return yes;
    }
    no
}

#[cfg(test)]
mod module_test {
    use super::*;

    #[test]
    fn _test_case1() {
        assert_eq!("Yes", resolve(3, 5))
    }

    #[test]
    fn _test_case2() {
        assert_eq!("No", resolve(16, 2))
    }

    #[test]
    fn _test_case3() {
        assert_eq!("No", resolve(12, 15))
    }
}


fn main() {
    input! {
        xy: (usize, usize)
    }
    let (x, y) = xy;
    println!("{}", resolve(x, y))
}
