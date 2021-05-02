use proconio::input;
fn main() {
    input! {
        mut s: u32,
    }
    let mut list = Vec::<u32>::new();
    list.push(s);
    let mut cnt = 0;
    for i in 1..=1_000_000 {
        if s % 2 == 0 {
            s = s/2;
            list.push(s);
        } else {
            s = 3 * s + 1;
            list.push(s);
        }
        if s == 1 {
            cnt += 1;
            if cnt > 1 {
                break;
            }
        }
    }
    let mut ans = 0;
    for i in 1..=list.len()-1 {
        for j in 0..=i-1 {
            if list[i] == list[j] {
                ans = i+1;
                println!("{}", ans);
                return;
            }
        }
    }
}
