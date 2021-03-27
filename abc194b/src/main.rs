use proconio::input;
use std::cmp;
fn main() {
    input! {
        n: usize,
        ab: [(usize, usize); n],
    }
    let mut worktime_one = vec![];
    let mut atime = vec![];
    let mut btime = vec![];

    for one in ab {
        worktime_one.push(one.0 + one.1);
        atime.push(one.0);
        btime.push(one.1);
    }
    worktime_one.sort();

    let mut worktime_two: usize = usize::max_value();
    for i in 0..n {
        for j in 0..n {
            if i != j {
                worktime_two = cmp::min(worktime_two, cmp::max(atime[i], btime[j]));
            }
        }
    }

    println!("{}", cmp::min(worktime_one[0], worktime_two));
}
