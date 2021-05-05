use proconio::input;
fn main() {
    input! {
        n: u32,
    }
    let s = format!("{:0>1$b}", n, 8);
    println!("{}", s);
    // guess 0b1000 & 0b1000 == 0b0000 -> NO
    println!("{}", if n&1<<3 == 0 {"YES"}else{"NO"});
}
