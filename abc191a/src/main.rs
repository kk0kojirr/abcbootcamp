use proconio::input;

fn main() {
    input! {
        mut v: f32,
        mut t: f32,
        mut s: f32,
        mut d: f32,
    }

    if t <= d / v && d / v <= s {
        println!("No");
    } else {
        println!("Yes");
    }
}
