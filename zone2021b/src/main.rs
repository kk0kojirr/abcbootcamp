use proconio::input;
fn main() {
    input! {
        n: usize,
        d: f32,
        h: f32,
        dhs: [(f32, f32); n],
    }
    let mut min = h / d;
    for dh in dhs {
        let ang = (h - dh.1) / (d - dh.0);
        if min > ang {
            min = ang;
        }
    }
    println!("{}", h - d * min);
}
