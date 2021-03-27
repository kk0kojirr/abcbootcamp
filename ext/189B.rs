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
    let nx: Vec<(u32, u32)> = (0..1).map(|_| (read(), read())).collect();
    let vp: Vec<(u32, u32)> = (0..nx[0].0).map(|_| (read(), read())).collect();

    // v ext 200 5
    let mut i: u32 = 0;
    let mut sum: u32 = 0;

    for v in vp {
        sum = sum + (v.0 * v.1);
        i = i + 1;        

        if sum > nx[0].1 * 100 {
            break;
        }        
    }

    if sum > nx[0].1 * 100 {
        println!("{}", i);
    }
    else
    {
        println!("{}", -1);
    }
}
