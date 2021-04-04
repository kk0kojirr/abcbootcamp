use proconio::input;
fn main() {
    input! {
        s: String,
    }
    let mut prev: char = ' ';
    for c in s.chars() {
        if prev == ' ' {
            prev = c;
        }

        if prev != ' ' && prev == c {

        } else {
            println!("Lost");
            return;
        }
    }
    println!("Won");
}
