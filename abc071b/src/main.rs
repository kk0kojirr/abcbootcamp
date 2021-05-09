use proconio::input;
fn main() {
    input! {
        mut s: String,
    }
    for i in b'a'..=b'z' {
        if !s.contains(i as char) {
            println!("{}", i as char);
            return
        }
    }
    println!("None");
}
