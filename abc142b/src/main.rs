use proconio::input;
fn main() {
    input! {
        n: i32,
        k: i32,
        h: [i32; n],
    }
    let mut ans = 0;
    for hh in h {
        if k <= hh {
            ans += 1;
        }
    }
    println!("{}", ans);
}
