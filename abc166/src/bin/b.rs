use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
    };
    let mut res: Vec<usize> = vec![0; n];

    (0..k).for_each(|_| {
        input! {
            d: usize,
            a: [usize; d],
        };

        a.iter().for_each(|i| {
            res[i - 1] += 1;
        });
    });

    println!("{}", res.iter().filter(|&&x| x == 0).count());
}
