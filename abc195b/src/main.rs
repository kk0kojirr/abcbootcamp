use proconio::input;
fn main() {
    input! {
        a: i32,
        b: i32,
        w: i32,
    }
    let mut min = std::i32::MAX;
    let mut max = 0;
    for i in 1..=1000000 {
        if a*i <= 1000*w && b*i >= 1000*w {
            min = std::cmp::min(min, i);
            max = std::cmp::max(max, i);
        }
    }
    if max == 0 {
        println!("UNSATISFIABLE");
    } else {
        println!("{} {}", min, max);
    }
}
