use proconio::input;
fn main() {
    input! {
        n: usize,
        xy: [(i32, i32); n],
    }
    let mut ans: u32 = 0;
    for i in 0..n {
        for j in i+1..n {
            let tilt = (xy[i].1 -xy[j].1) as f64 / (xy[i].0 -xy[j].0) as f64;
            if tilt.abs() <= 1.0 {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}
