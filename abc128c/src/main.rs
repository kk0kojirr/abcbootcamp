use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        // switches
        n: usize,
        // lights
        m: usize,
    }

    let mut s = vec![];

    // all lights atache
    for _ in 0..m {
        input! {
            // the lisght switches
            // k個に接続
            k: usize,
            // the light switch atach number
            ss: [usize; k]
        }
        s.push(ss);
    }
    // the status of the light on
    input! {
        p: [usize; m]
    }

    // 全点灯の組み合わせの個数
    let mut ans = 0;

    // onであるスイッチの部分集合の列挙 0から2のn(switches)乗
    for switchonbit in 0..2u32.pow(n as u32) {
        let mut all_is_on = true;

        // ライト分のループ
        for k in 0..m {
            let mut on_num = 0;

            for connectedid in &s[k] {
                let connectedid = connectedid - 1;

                // the lightに接続されているswitchがonか？
                if switchonbit & (1 << connectedid) > 0 {
                    on_num += 1;
                }
            }

            // pによって不点灯の状態があり
            if on_num % 2 != p[k] {
                all_is_on = false;
            }
        }

        if all_is_on {
            ans += 1;
        }
    }
    println!("{}", ans);
}
