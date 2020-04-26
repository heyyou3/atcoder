use proconio::input;

fn main() {
    input! {
        n: String,
    };
    let res = if n.chars().find(|x| x.to_string() == "7") != None {
        "Yes"
    } else {
        "No"
    };
    println!("{}", res);
}
