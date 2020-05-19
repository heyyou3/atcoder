use proconio::input;

fn main() {
    input! {
      a: f64,
      b: f64,
      h: f64,
      m: f64,
    };
    // 針がそれぞれ1周の何％動いているかを求める
    let h: f64 = (h * 60.0 + m) / 720.0;
    let m: f64 = m / 60.0;
    // 差から 1 周の何％分開いているかを求める。(小さいほうの角度を使う)
    let p: f64 = (h - m).abs();
    let p = p.min(1.0 - p);
    // 角度にする
    let ang = p * 360.0;
    // ラジアンに変換する
    let rad: f64 = ang * std::f64::consts::PI / 180.0;
    // 余弦定理を使用する
    let ans = (a * a) + (b * b) - 2.0 * a * b * rad.cos();
    println!("{}", ans.sqrt());
}
