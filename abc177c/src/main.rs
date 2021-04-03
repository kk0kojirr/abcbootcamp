use proconio::input;
fn main() {
    input! {
        n: usize,
        a: [u32; n],
    }

    println!("{}", a.iter().fold(0, |sum, i| sum + i));

    let mut ans: u128 = 0;
    for i in 0..n {
        for j in 0..n {
            if j <= i {
                continue;
            }
            ans += a[i] as u128 * a[j] as u128;
        }
    }

    ans = ans % (1_000_000_000 + 7);
    println!("{}", ans);
}
