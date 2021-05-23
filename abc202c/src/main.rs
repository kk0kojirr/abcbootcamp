use proconio::input;
use std::collections::HashMap;
fn main() {
    input! {
        n: usize,
        a: [i64; n],
        b: [i64; n],
        c: [i64; n],
    }
    let mut d = HashMap::<i64,u64>::new();
    for cc in c {
        if d.get(&b[cc as usize -1]) == None {
            d.insert(b[cc as usize -1], 1);
        } else {
            d.insert(b[cc as usize -1], d.get(&b[cc as usize -1]).unwrap() + 1);
        }
    }

    let mut ans = 0;
    for i in a {
        if d.get(&i) == None {
        } else {
            ans += d.get(&i).unwrap();
        }
    }
    println!("{}", ans);
}
