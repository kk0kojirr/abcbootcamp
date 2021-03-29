use proconio::input;
fn main() {
    input!{
        d: u32,
        t: u32,
        s: u32,
    }
    println!("{}", if s * t >= d {"Yes"}else{"No"});
}
