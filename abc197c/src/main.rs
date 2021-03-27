use proconio::input;
fn main() {
    input!{
        n: u32,
        a: [u32; n],
    }
    let mut ans: u32 = std::u32::MAX;
    for i in 0..(1 << (n - 1)) {
        let mut or = 0;
        let mut xor = 0;
        for (j, a) in a.iter().enumerate() {
            or = or | a;
            if i >> j & 1 == 0 {
                xor = xor ^ or;
                or = 0;
            }
        }
        xor = xor ^ or;
        ans = std::cmp::min(ans, xor);
    }
    println!("{}", ans);
}
