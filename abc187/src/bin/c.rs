use proconio::input;

fn resolve(s: Vec<String>) -> String {
    let mut hash_set = std::collections::HashSet::new();

    for text in s {
        if &text[0..1] == "!" {
            let body = &text[1..];

            if hash_set.contains(body) {
                return body.to_string();
            }
        } else {
            if hash_set.contains(&format!("!{}", text)) {
                return text;
            }
        }
        hash_set.insert(text);
    }

    "satisfiable".to_string()
}

#[cfg(test)]
mod module_test {
    use super::*;

    #[test]
    fn _test_case1() {
        let test_data = vec!["a", "!a", "b", "!c", "d", "!d"].into_iter().map(|s| s.to_string()).collect();
        assert_eq!("a", resolve(test_data));
    }

    #[test]
    fn _test_case2() {
        let test_data = vec!["red", "red", "red", "!orange", "yellow", "!blue"].into_iter().map(|s| s.to_string()).collect();
        assert_eq!("satisfiable", resolve(test_data));
    }

    #[test]
    fn _test_case3() {
        let test_data = vec!["red"].into_iter().map(|s| s.to_string()).collect();
        assert_eq!("satisfiable", resolve(test_data))
    }
}


fn main() {
    input! {
        n: usize,
        s: [String; n],
    }
    println!("{}", resolve(s))
}
