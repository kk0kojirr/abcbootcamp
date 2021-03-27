use proconio::input;
fn main() {
    input! {
        c: char,
    }
    match c {
        'a' | 'i' | 'u' | 'e' | 'o' => println!("vowel"),
        _ => println!("consonant"),
    } 
}
