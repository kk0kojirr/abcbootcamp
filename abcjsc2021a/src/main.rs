use proconio::input;
fn main() {
    input! {
        x: u32,
        y: u32,
        z: u32
    }
    let g: f64 = y as f64 / x as f64;
    let t = g * z as f64;
    let s = if t - 1.0 < 0.0 {0} else {(t - 1.0).ceil() as u32};
    println!("{}", s);
}
