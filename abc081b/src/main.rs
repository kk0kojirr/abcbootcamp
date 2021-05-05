use proconio::input;
fn main() {
    input! {
        n: u32,
        mut a: [u32; n],
    }
    let mut aa:Vec<u32> = a.into_iter().map(|v| v).collect();

    for i in 0.. {
        for aa in &aa {
            if aa % 2 == 1 {
                println!("{}", i);
                return;
            }
        }
        aa = aa.into_iter().map(|v| v/2).collect();
    }
}
