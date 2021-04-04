use proconio::input;
fn main() {
    input! {
        n: usize,
        a: [u32; n],
    }
    let mut ans: u32 = 0;
    for left in 0..n {
        let mut tmp = a[left];
        for right in left..n {
            tmp = std::cmp::min(tmp, a[right]);
            ans = std::cmp::max(ans, (right-left+1)as u32 * tmp);
        }
    }
    println!("{}", ans);
}
