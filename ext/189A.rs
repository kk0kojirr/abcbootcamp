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
    let s: Vec<char> = read::<String>().chars().collect();

    if (s[0] == s[1]) & (s[1] == s[2]) {
        println!("{}", "Won");
    } else {
        println!("{}", "Lost");
    }
}
