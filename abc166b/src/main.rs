use proconio::input;
fn main() {
    input! {
        n: usize,
        k: u32,
    }
    let mut treat = vec![0; n];
    for _i in 0..k {
        input! {
            mut d: u32,
            mut a: [u32; d],
        }
        for v in a {
            treat[(v - 1) as usize] += 1;
        }
    }
    let mut ans: u32 = 0;
    for treat in treat {
        if treat == 0 {
            ans += 1;
        }
    }
    println!("{}", ans);
}
