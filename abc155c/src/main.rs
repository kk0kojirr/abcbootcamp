use proconio::input;
use std::collections::HashMap;
fn main() {
    input! {
        n: usize,
        mut s: [String; n],
    }
    let mut dic = HashMap::new();
    for ss in &s {
        if !dic.contains_key(&ss) {
            dic.insert(ss, 1);
        } else {
            dic.insert(ss, dic[&ss]+1);
        }
    }
    let mut max = 0;
    for (key, val) in dic.iter() {
        max = std::cmp::max(max, *val);
    }
    let mut vec = Vec::new();
    for (kk, &vv) in dic.iter() {
        if vv == max {
            vec.push(kk);
        }
    }
    vec.sort();
    for s in vec {
        println!("{}", s);
    }
}
