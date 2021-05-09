use proconio::input;
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut cnt = vec![0;200];
    let mut ans:u128 = 0;

    for a in a {
        let x = a % 200;
        ans += cnt[x];
        cnt[x] += 1;
    }
    println!("{}", ans);
}
