use proconio::input;

fn main() {
    input! {
        n: u64,
    }
    let mut d = vec![];
    for b in 2.. {
        if 2u64.pow(b) > n {
            break;
        }
        d.extend((2u64..).take_while(|a| a.pow(b) <= n).map(|a| a.pow(b)));
    }
    d.sort();
    d.dedup();
    let ans = n as usize - d.len();
    println!("{}", ans);
}
