use proconio::input;
fn main() {
    input!{
        n: u32,
        a: [u32; n],
    }
    let mut ans: u32 = std::u32::MAX;
    for sep in 1..a.len() {
        let mut cur: u32 = 0;

        let mut aaor: u32 = a[0]; 
        for aa in 1..sep {
            aaor = aaor | a[aa];
        }
        // ------
        let mut abor: u32 = a[sep];
        for ab in sep+1..a.len() {
            abor = abor | a[ab];
        }

        cur = aaor ^ abor;
        ans = std::cmp::min(ans, cur);
    }

    println!("{}", ans);
}
