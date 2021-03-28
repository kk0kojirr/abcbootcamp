`use proconio::input;
fn main() {
    input! {
        h: usize,
        w: usize,
        a: [u32; h*w],
    }
    let min = a.iter().min().unwrap();
    let ans = a.iter().map(|v| v - min).sum::<u32>();
    println!("{}", ans);
}
