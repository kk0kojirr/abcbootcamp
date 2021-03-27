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
    let n: u32 = read();
    let x: Vec<i32> = (0..n).map(|_| read()).collect();
    let y: Vec<i32> = (0..n).map(|_| read()).collect();

    let mut sum: i32 = 0;
    for i in 0..n {
        sum += x[i as usize] * y[i as usize];
    }

    if sum == 0
    {
        println!("Yes");
    }
    else
    {
        println!("No");
    }
}
