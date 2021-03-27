use std::io::*;
use std::str::FromStr;

fn read<T: FromStr>() -> T {
    let stdin = stdin();
    let stdin = stdin.lock();
    let token: String = stdin
        .bytes()
        .map(|c| c.expect("failed to read char") as char) 
        .skip_while(|c| c.is_whitespace())
        .take_while(|c| !c.is_whitespace())
        .collect();
    token.parse().ok().expect("failed to parse token")
}

fn main() {
    let a: u32 = read();
    let b: u32 = read();
    let c: u32 = read();
    let x: u32 = read();
    let mut ans = 0;

    for i in 0..a + 1 {
        for j in 0..b + 1 {
            for k in 0..c + 1 {
                if i * 500 + j * 100 + k * 50 == x {
                    ans += 1;
                }
            }
        }
    }

    println!("{}", ans);
}