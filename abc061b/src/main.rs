use proconio::input;
fn main() {
    input! {
        n: usize,
        m: u32,
        ab: [(usize, usize); m],
    }
    let mut load = vec![0; n];
    for (a, b) in ab {
        load[a-1] += 1;
        load[b-1] += 1;
    }
    for l in load {
        println!("{}", l);
    }
}
