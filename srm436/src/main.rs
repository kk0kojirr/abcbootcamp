use proconio::input;
use std::cmp::max;

fn main() {
    input! {
        n: usize,
        fs: [String; n],
    }

    let mut ans: u32 = 0;
    for (i, f) in fs.iter().enumerate() {
        let mut cnt: u32 = 0;

        // i is index, c is char
        for (j, c) in f.chars().enumerate() {
            if i == j {
                continue;
            }

            if c == 'Y' {
                cnt += 1;
            } else {
                for k in 0..(n-1) {
                    if fs[j].chars().collect::<Vec<char>>()[k] == 'Y' &&
                        fs[k].chars().collect::<Vec<char>>()[i] == 'Y' {
                        cnt += 1;
                    }
                }
            }
        }

        ans = max(ans, cnt);
    }
    println!("{}", ans);
}
