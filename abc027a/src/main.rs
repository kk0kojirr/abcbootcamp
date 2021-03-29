use proconio::input;
fn main() {
    input! {
        n: u32,
        mut x: u32,
        mut a: [u32; n]
    }
    a.sort();
    let mut ans: u32 = 0;
    for a in a {
        if a > x {
            x = 0;
            break;
        } else {
            ans += 1;
            x -= a;
        }
    }
    if x > 0 {ans -= 1}
    println!("{}", ans);
}
