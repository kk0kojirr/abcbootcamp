use proconio::input;
use std::cmp::max;

fn main() {
    input! {
        n: usize,
        first: [String; n],
        seccond: [String; n],
    }

    // map filtermaptest
    // println!("{:?}", (0..=20).filter(|n| n%2==1).map(|n|n*n).collect::<Vec<u32>>());
    // (0..=20)
    //     .filter_map(|n| if n % 2 == 1 { Some(n * n) } else { None })
    //     .for_each(|n| print!("{}, ", n));

    let mut ans: u32 = 0;

    for i in 0..n {
        let mut f: u32 = 0;
        let mut s: u32 = 0;

        for j in 0..n {
            if first[i] == first[j] {
                f += 1;
            }
            if first[i] == seccond[j] {
                f += 1;
            }
            if seccond[i] == first[j] {
                s += 1;
            }
            if seccond[i] == seccond[j] {
                s += 1;
            }
        }

        ans = max(f, ans);
        ans = max(s, ans);
    }

    println!("{}", ans);
}
