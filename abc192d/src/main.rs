use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        x: Chars,
        m: i64,
    }
    let f = x.iter().map(|i| i.to_digit(10).unwrap() as i64).collect::<Vec<i64>>();
    let ff = *f.iter().max().unwrap();

    if f.len() == 1 {
        if f[0] <= m {
            println!("{}", 1);
        } else {
            println!("{}", 0);
        }
        return;
    }
    
    let mut ng = m + 1;
    let ok = ff;

    while ng - ok > 1 {
        let mid = (ng + ok) / 2;
        let mut z = 0;
        for &x in &f {
            if z > (m - x) / mid {
                z = m + 1;
                break;
            }
            z = z * mid + x;
        }
        if z > m {
            ng = mid;
        }
    }
    println!("{}", std::cmp::max(0, ok - ff));
}
