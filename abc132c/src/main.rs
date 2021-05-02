use proconio::input;
fn main() {
    input!{
        n: usize,
        mut d: [u32; n],
    }
    d.sort();
    let mut arc = vec![false; n];
    let k = d[n/2-1];
    let kp = d[n/2];
    let diff = kp - k;
    for (i, &dd) in d.iter().enumerate() {
        if dd >= kp {
            arc[i] = true;
        } else {
            arc[i] = false;
        }
    }
    if arc.iter().filter(|&x| *x).count() == n/2 {
        println!("{}", diff);
    } else {
        println!("0");
    }
}
