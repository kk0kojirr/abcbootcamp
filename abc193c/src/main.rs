use proconio::input;

fn main() {
    input! {
        n: u128,
    }

    let mut num = vec![false; n as usize+1];

    let mut maxpow: u32 = 2;
    let mut test: u128 = 0;

    while test < n {
        test = 2u128.pow(maxpow);
        if test > n {
            break;
        }
        num[test as usize] = true;
        maxpow = maxpow + 1;
    }

    for i in 3..=n/2 {
        for j in 2..=maxpow {
            let a = i.pow(j);

            if a <= n {
                num[a as usize] = true;
            } else {
                break;
            }
        }
    }

    let mut ans: u64 = 0;
    for t in num {
        if !t {
            ans += 1;
        }
    }

    println!("{}", ans-1);
}
