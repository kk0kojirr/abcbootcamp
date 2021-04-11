use proconio::input;
fn main() {
    input! {
        n: usize,
        a: [u32; n],
    }
    let mut max = 0;
    let mut res = -1;
    for i in 2..=1000 {
        let mut cnt = 0;
        for j in 0..n {
            if max < cnt {
                max = cnt;
                res = i;
            }
        }
    }
    println!("{}", ans);
}
