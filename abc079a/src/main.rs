use proconio::input;
use proconio::marker::Usize1;
fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(Usize1, Usize1); m],
    }

    let mut isgoal = vec![false;n];
    for (from, to) in ab.iter() {
        if *to == n -1 {
            isgoal[*from] = true;
        }
    }

    for (from, to) in ab.iter() {
        if *from == 0 && isgoal[*to] {
            println!("{}", "POSSIBLE");
            return;
        }
    }

    println!("{}", "IMPOSSIBLE");
}
