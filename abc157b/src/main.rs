use proconio::input;
fn main() {
    input! {
        a: [[u32; 3]; 3],
        n: usize,
        b: [u32; n],
    }
    let mut bingo: [[u8; 3]; 3] = [
        [0,0,0],
        [0,0,0],
        [0,0,0]
    ];
    for ok in 0..n {
        for x in 0..3 {
            for y in 0..3 {
                if a[x][y] == b[ok] {
                    bingo[x][y] = 1;
                }
            }
        }
    }

    for i in 0..3 {
        if bingo[0][i] + bingo[1][i] + bingo[2][i] == 3 {
            println!("Yes");
            return;
        }
        if bingo[i][0] + bingo[i][1] + bingo[i][2] == 3 {
            println!("Yes");
            return;
        }
    }
    if bingo[0][0] + bingo[1][1] + bingo[2][2] == 3 {
        println!("Yes");
        return;
    }
    if bingo[2][0] + bingo[1][1] + bingo[0][2] == 3 {
        println!("Yes");
        return;
    }

    println!("No");
}
