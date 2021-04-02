use proconio::input;
fn main() {
    input! {
        k: u32,
        a: u32,
        b: u32,
    }
    for i in a..=b {
        if i % k == 0 {
            println!("{}", "OK");
            return
        }
    }
    println!("{}", "NG");
}
