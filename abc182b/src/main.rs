use proconio::input;
fn main() {
    input! {
        n: usize,
        a: [u32; n],
    }
    let mut max = 0;
    let mut res = 0;
    for i in 2..=1000 {
        let mut cnt = 0;
        for num in &a {
            if num % i == 0 {
                cnt += 1;
            }
            if max < cnt {
                max = cnt;
                res = i;
            }
        }
    }
    println!("{}", res);
}
