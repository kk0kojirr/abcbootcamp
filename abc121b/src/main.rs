use proconio::input;
fn main() {
    input! {
        n: usize,
        m: usize,
        c: i32,
        b: [i32; m],
        a: [[i32; m]; n],
    }
    let mut sum: i32 = 0;
    let mut ans: u32 = 0;
    for i in 0..n {
        sum = 0;
        for j in 0..m {
            sum += a[i][j] * b[j];
        }
        sum += c;
        if sum > 0 {
            ans += 1;
        }
    }
    println!("{}", ans);
}
