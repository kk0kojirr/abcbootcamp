use proconio::input;
fn main() {
    input! {
        k: i32,
        n: u32,
        mut a: [i32; n]
    }
    let mut prev: i32 = 0;
    let mut max: i32 = 0;
    for w in a.windows(2) {
        max = std::cmp::max(prev, w[1] - w[0]);
        prev = w[1] - w[0];
    }
    println!("{}", k - max);
}
