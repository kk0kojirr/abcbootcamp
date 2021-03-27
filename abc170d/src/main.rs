use proconio::input;

fn main() {
    input! {
        n: u32,
        a: [u32; n],
    }
    
    let m = 1000000;
    let mut cnt = vec![0; m+1];
    for a in a {
        cnt[a] += 1;
    }
    let mut ans = 0;
    for i in 1..=m {
        if cnt[i] == 0 {
            continue;
        }
        if cnt[i] == 1 {
            ans+= 1;
        }
        let mut j = i;
        while let Some(c) = cnt.get_mut(j) {
            *c = 0;
            j += i;
        }
    }

    println!("{}", ans);
}
