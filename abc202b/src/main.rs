use proconio::input;
fn main() {
    input! {
        mut s: String,
    }
    for c in s.chars().rev() {
        if c == '6' {
            print!("9");
        } else if c == '9' {
            print!("6");
        } else {
            print!("{}", c);
        }
    }
    println!("");
}
