use proconio::input;
fn main() {
    input! {
        n: usize,
        a: [usize; n],
        b: [u32; n],
        c: [u32; n-1],
    }
    let mut prev = 100;
    let mut ans = 0;
    for i in a {
        ans += b[i-1];
        if i == prev+1 {
            ans += c[i-2];
        }
        prev = i;
    }
    println!("{}", ans);
}
