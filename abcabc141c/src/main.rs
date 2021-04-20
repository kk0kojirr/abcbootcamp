use proconio::input;
fn main() {
    input! {
        n: usize,
        k: i64,
        q: usize,
        a: [i64; q],
    }
    let mut pt = vec![k - q as i64; n];
    for i in a {
        pt[i as usize -1] += 1;
    }
    for p in pt {
        if p > 0 {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
