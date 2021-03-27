use proconio::input;
fn main() {
    input! {
        n: i64,
    }

    let mut cnt: u64 = 0;

    for i in 1..1000000 {
        if i > n {
            break;
        }
        let s = format!("{}{}", i, i);
        let x: i64 = s.parse().unwrap();
        if x >= 1 && x <= n {
            cnt += 1;
        }
    }

    println!("{}", cnt);
}
