use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
    };

    let mut cnt = 0;

    for j in 0..h-1 {
        for i in 0..w-1 {
            let snn = s[j][i];
            let snp = s[j][i+1];
            let spn = s[j+1][i];
            let spp = s[j+1][i+1];
            let c = match (snn, snp, spn, spp) {
                ('.','#','#','#') => 1,
                ('#','.','#','#') => 1,
                ('#','#','.','#') => 1,
                ('#','#','#','.') => 1,
                ('#','.','.','.') => 1,
                ('.','#','.','.') => 1,
                ('.','.','#','.') => 1,
                ('.','.','.','#') => 1,
                _ => 0,
            };
            cnt += c;
        }
    }

    println!("{}", cnt);
}
