use proconio::input;
fn main() {
    input!(s: String);
    for (i, c) in s.chars().enumerate() {
        if i == s.len()-1 {
            if c == 's' {
                println!("{}{}", c, "es");
            } else {
                println!("{}{}", c, "s");
            }
        } else {
            print!("{}", c);
        }
    }
}
