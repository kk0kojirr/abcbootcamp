use proconio::input;
fn main() {
    input! {
        n: usize,
        d: u32,
        x: u32,
        a: [u32; n],
    }
    let mut ans = 0;
    for i in a {
        for day in 1..=d {
            for j in 0..=d as u32 {
                if day == i * j + 1 {
                    ans += 1;
                }
            }
        }
    }
    println!("{}", ans + x);
}
