use proconio::input;
use proconio::marker::Chars;
fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
    }
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '.' {
                continue;
            }
            if (i == 0 || s[i-1][j] == '.') &&
                (j == 0 || s[i][j-1] == '.') &&
                (i == h-1 || s[i+1][j] == '.') &&
                (j == w-1 || s[i][j+1] == '.') {
                    println!("No");
                    return;
            }
        }
    }
    println!("{}", "Yes");
}
