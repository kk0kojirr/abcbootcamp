use proconio::input;
fn main() {
    input! {
        a: u32,
        b: u32,
    }
    let mut outlet: u32 = 1;
    let mut ans: u32 = 0;
    while outlet < b {
        outlet -= 1;
        outlet += a;
        ans += 1;
    }

    println!("{}", ans);
}
