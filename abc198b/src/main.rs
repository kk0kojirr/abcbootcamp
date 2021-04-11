use proconio::input;
use proconio::marker::Chars;
fn main() {
    input! {
        n: Chars,
    }

    let mut revs = n.iter().rev().collect::<String>();
    let mut ords = n.iter().collect::<String>();
    let mut ordszero = 0;

    let mut zeros = 0;
    for c in revs.chars() {
        if c == '0' {
            zeros += 1;
        } else {
            break;
        }
    }

    if revs == ords {
        println!("Yes");
        return;
    } else if format!("{}{}", "0".repeat(zeros), ords) == format!("{}{}", revs, "0".repeat(zeros)) {
        println!("Yes");
        return;
    } else {
        println!("No");
        return;
    }
}
