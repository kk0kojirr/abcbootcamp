use proconio::input;

fn main() {
    input! {
        n: usize,
        lrs: [(u64, u64); n],
    }

    for lr in lrs {
        if 2*lr.0 <= lr.1 {
            let n = lr.1 - 2 * lr.0 + 1;
            println!("{}", n*(n+1)/2);
        } else {
            println!("0");
        }
    }
}
