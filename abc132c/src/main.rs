use proconio::input;
fn main() {
    input!{
        n: usize,
        mut d: [u32; n],
    }
    d.sort();
    let mut arc = vec![false; n];
    let k = d[n/2];
    for (i, &dd) in d.iter().enumerate() {
        if dd >= k {
            arc[i] = true;
        } else {
            arc[i] = false;
        }
    }
    println!("{:?},{:?}||{} {}", arc, d, k, arc.iter().filter(|&x| *x).count());
}
