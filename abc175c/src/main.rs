use proconio::input;
fn main() {
    input! {
        mut x: i128,
        mut k: i128,
        d: i128,
    }
    let mut x = x.abs();
    let straight = std::cmp::min(k, x/d);
    k -= straight;
    x -= straight*d;
    println!("{}", if k % 2 == 0 {x} else {d-x});
}
