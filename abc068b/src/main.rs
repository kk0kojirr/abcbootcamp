fn main() {
    proconio::input!{n: u32}

    let mut ans: u32 = 1;
    let mut cnt: u32 = 0;
    for num in 1..=n {
        let mut tmp = num;
        let mut tmpcnt = 0;
        while tmp % 2 == 0 {
            tmp /= 2;
            tmpcnt += 1;
        }
        if tmpcnt > cnt {
            cnt = tmpcnt;
            ans = num;
        }
    }
    println!("{}", ans);
}
