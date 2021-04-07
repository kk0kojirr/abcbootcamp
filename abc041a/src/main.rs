use proconio::input;
fn main() {
    input! {
        s: String,
        i: usize,
    }
    println!("{}", s.chars().collect::<Vec<char>>()[i-1]);
}
