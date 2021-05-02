use proconio::input;
use std::collections::VecDeque;
fn main() {
    input!{
        n: usize,
        mut v: [u32; n],
    }
    let mut vque = VecDeque::<f32>::new();
    v.sort();
    for vv in v {
        vque.push_back(vv as f32);
    }

    while let Some(vv) = vque.pop_front() {
        let vvv = vque.pop_front();
        if vvv == None {
            break;
        }
        vque.push_front((vv+vvv.unwrap())/2.0);
        if vque.len() == 1 {
            break;
        }
    }
    println!("{}", vque.pop_back().unwrap());
}
