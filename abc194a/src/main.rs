use proconio::input;
fn main() {
    input! {
        a: u32,
        b: u32,
    }

    let mut ans: u8 = 0;
    if a + b >= 15 && b >= 8 {
        ans = 1;
    } else if a + b >= 10 && b >= 3 {
        ans = 2;
    } else if a + b >= 3 {
        ans = 3;
    } else {
        ans = 4;
    }
    println!("{}", ans);
}
