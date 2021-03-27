use proconio::input;

fn main() {
    input! {
        n: usize,
        m: u32,
        ab: [(usize, usize); m],
        k: u32,
        cd: [(usize, usize); k],
    }


    let mut cnt: u32 = 0;

    let mut tmp: u32 = 0;
    for onbit in 0..2u32.pow(k as u32) {
        let mut dish = vec![0; n];

        tmp = 0;
        let mut i : u32 = 0;
        for (c, d) in &cd {
            if onbit & (1 << i) > 0 {
                dish[c - 1] += 1;
            } else {
                dish[d - 1] += 1;
            }
            i += 1;
        }

        for (a, b) in &ab {
            if dish[a - 1] >= 1 && dish[b - 1] >= 1 {
                tmp += 1;
            }
        }

        if tmp > cnt {
            cnt = tmp;
        }
    }

    println!("{}", cnt);
}
