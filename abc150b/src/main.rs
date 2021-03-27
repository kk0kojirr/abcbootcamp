use proconio::input;
use proconio::marker::{Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
    }

    let mut cnt : u32 = 0;
    for i in 0..n {
        if (i + 2 < n) {
            if s[i as usize] == 'A' && s[i+1] == 'B' && s[i+2] == 'C' {
                cnt = cnt + 1;
            }
        }
    }

    println!("{}", cnt);
}
