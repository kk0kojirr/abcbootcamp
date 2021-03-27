fn main() {
    proconio::input!(h: usize, w: usize, a: usize, b: usize);
    let mut ans = 0;
    let mut dfs = vec![(0, 0, a, b, 0)];
    
    while let Some((x, y, a, b, s)) = dfs.pop() {

        println!("{}:{}, {}:{}  {:08b}:::{}", x, y, a, b, s, ans);
        println!("s ^ (1 << y) {:08b}", s^(1 << y));
        println!("{:08b}", s >> (y + 1) & 1);
        if x == h {
            ans += 1;
        } else if y == w {
            dfs.push((x + 1, 0, a, b, s));
        } else if s >> y & 1 == 1 {
            dfs.push((x, y + 1, a, b, s ^ (1 << y)));
        } else {
            if b > 0 {
                dfs.push((x, y + 1, a, b - 1, s));
            }
            if a > 0 {
                if y + 1 < w && s >> (y + 1) & 1 == 0 {
                    dfs.push((x, y + 2, a - 1, b, s));
                }
                if x + 1 < h {
                    dfs.push((x, y + 1, a - 1, b, s | (1 << y)));
                }
            }
        }
    }
    println!("{}", ans);
}
