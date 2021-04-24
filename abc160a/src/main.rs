use proconio::input;
fn main() {
    input! {s: String,}
    let chars: Vec<char> = s.chars().collect();
    if chars[2] == chars[3] && chars[4] == chars[5] {
            println!("Yes");
    } else {
        println!("No");
    }
}
