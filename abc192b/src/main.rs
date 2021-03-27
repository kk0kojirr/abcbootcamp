use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    };

    let mut cnt: u32 = 1;
    let mut ans: String = String::from("Yes");

    for c in s {
        if cnt % 2 == 1 {
            if c.is_uppercase() {
                ans = String::from("No");
                break;
            }
        } else {
            if c.is_lowercase() {
                ans = String::from("No");
                break;
            }
        }
        cnt += 1;
    }

    println!("{}", ans);
}
