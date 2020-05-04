use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        h: [usize; n],
        ab: [(Usize1, Usize1); m],
    };
    let mut ans = vec![true; n];

    for (a, b) in ab {
        if h[a] <= h[b] {
            ans[a] = false;
        }
        if h[a] >= h[b] {
            ans[b] = false;
        }
    }

    println!("{}", ans.into_iter().filter(|&x| x).count());
}

