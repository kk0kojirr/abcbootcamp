use proconio::input;

fn main() {
    input! {
        n: u64,
        k: u32,
    }

    println!("{}", test(n, k));
}

fn test(n: u64, k: u32) -> u64 {
    if k == 0 {
        return n;
    } else {
        let mut c: Vec<char> = n.to_string().chars().collect();
        c.sort_by(|a, b| b.cmp(a));
        let dai_n: u64 = c.iter().collect::<String>().parse().unwrap();

        c.sort_by(|a, b| a.cmp(b));
        let syo_n: u64 = c.iter().collect::<String>().parse().unwrap();

        test(dai_n - syo_n, k - 1)
    }
}
