use proconio::input;
fn main() {
    input! {
        n: usize,
        m: usize,
        x: usize,
        a: [usize; m],
    }
    let mut current = x;
    let mut go = 0;
    loop {
        current += 1;
        if a.contains(&current) {
            go += 1;
        }
        if current == n {
            break;
        }
    }
    current = x;
    let mut back = 0;
    loop {
        current -= 1;
        if a.contains(&current) {
            back += 1;
        }
        if current == 0 {
            break;
        }
    }
    let ans = std::cmp::min(go, back);
    println!("{}", ans);
}
