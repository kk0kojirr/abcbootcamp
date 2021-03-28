use proconio::input;
fn main() {
    input! {
        h: usize,
        w: usize,
        mut x: usize,
        mut y: usize,
        s: [String; h]
    }
    x -= 1;
    y -= 1;
    let mut ans: u8 = 1;
    
    let mut lstop: bool = false;
    let mut rstop: bool = false;
    for i in 1..w {
        if y +i < w && !lstop {
            if s[x].chars().nth(y+i).unwrap() == '.' {
                ans += 1;
            } else {
                lstop = true;
            }
        }
        if y as i32 -i as i32 >= 0 && !rstop {
            if s[x].chars().nth(y-i).unwrap() == '.' {
                ans += 1;
            } else {
                rstop = true;
            }
        }
    }

    let mut tstop: bool = false;
    let mut bstop: bool = false;
    for j in 1..h {
        if x+j < h && !tstop {
            if s[x+j].chars().nth(y).unwrap() == '.' {
                ans += 1;
            } else {
                tstop = true;
            }
        }
        if x as i32 -j as i32 >= 0 && !bstop {
            if s[x-j].chars().nth(y).unwrap() == '.' {
                ans += 1;
            } else {
                bstop = true;
            }
        }
    }
    println!("{}", ans);
}
