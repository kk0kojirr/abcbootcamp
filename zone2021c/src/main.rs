use proconio::*;
 
fn main() {
    input! {
        n: usize,
        p: [[u32; 5]; n],
    }
    let mut ok = 1;
    let mut ng = 1_000_000_000 + 1;

    while ng - ok > 1 {
        let mid = (ok + ng) / 2;
        let mut cnt = [0; 1 << 5];
        for p in p.iter() {
            let mut bit = 0;
            for (i, &a) in p.iter().enumerate() {
                if a >= mid {
                    bit |= 1 << i;
                }
            }
            cnt[bit] += 1;
        }

        let mut v = vec![];
        for (i, c) in cnt.iter().enumerate() {
            for _ in 0..(*c).min(3) {
                v.push(i);
            }
        }

        let mut elem = false;
        for i in 0..v.len() {
            for j in 0..i {
                for k in 0..j {
                    elem |= v[i] | v[j] | v[k] == (1 << 5) - 1;
                }
            }
        }

        if elem {
            ok = mid;
        } else {
            ng = mid;
        }
    }
    println!("{}", ok);
}
