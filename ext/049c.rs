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
    let patterns: Vec<<Vec<char>>> = ["dream", "dreamer", "erase", "eraser"]
    .iter()
    .map(|s| s.chars().rev.collect())
    .collect();

    let s: Ves<char> = read::<String>().chars().rev().collect();
    let mut s = &s[..];
    let mut succeeded = true;
    while s.len() > 0 {
        let matched = patterns.iter().find(|&p| s.starts_with(p));
        if let Some(p) = matched {
            s = &s[p.len()..];
        } else {
            succeeded = false;
            break;
        }
    }

    println!("{}", if succeeded {"YES"} else {"NO"});
}