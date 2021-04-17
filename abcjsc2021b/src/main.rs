use proconio::input;
fn main() {
    input! {
        n: usize,
        m: usize,
        mut a: [u32; n],
        mut b: [u32; m],
    }

    let mut ans = Vec::<u32>::new(); 
    for i in 0..1010 {
        if a.contains(&i) && b.contains(&i) {
            continue;
        } else if a.contains(&i) || b.contains(&i) {
            ans.push(i);
            continue;
        } else {
            continue;
        }
    }

    for n in ans {
        print!("{} ", n);
    }
    println!("");
}
