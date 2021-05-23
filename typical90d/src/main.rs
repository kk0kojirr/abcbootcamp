use proconio::input;
fn main() {
    input! {
        h: usize,
        w: usize,
        a:[[u64;w];h],
    }
    let mut aa = vec![vec![0u64; w]; h];//<Vec<Vec<u32>>>::new();
    // println!("{:?}   {:?}", a, aa);
    let mut linetotal = vec![0u64; h];
    for i in 0..h {
        linetotal[i] = a[i].iter().sum();
    }
    let mut columntotal = vec![0u64; w];
    for i in 0..w {
        let mut tmp = 0;
        for k in 0..h {
            tmp += a[k][i]
        }
        columntotal[i] = tmp;
    }
    for i in 0..h {
        for j in 0..w {
            aa[i][j] = linetotal[i] + columntotal[j] - a[i][j];
        }
    }
    for aaa in aa {
        for aaaa in aaa {
            print!("{} ", aaaa);
        }
        println!("");
    }
}
