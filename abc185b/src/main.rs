use proconio::input;
fn main() {
    input! {
        n: i64,
        m: usize,
        t: u32,
        mut ab: [(u32, u32); m],
    }
    ab.reverse();
    let mut cur: i64 = n;
    let mut rest = ab.pop().unwrap();
    for t in 1..=t {
        if t > rest.0 && t <= rest.1 {
            if cur < n {
                cur += 1;
            }
            if t == rest.1 {
                let tmp = ab.pop();
                if tmp != None {
                    rest = tmp.unwrap();
                }
            }
        } else {
            cur -= 1;
        }
        //println!("{}: {}  {:?}",t, cur, rest);

        if cur <= 0 {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
