use proconio::input;
fn main() {
    input! {
        a: u32,
        b: u32,
        m: u32,
        mut aa: [u32; a],
        mut bb: [u32; b],
        xyc: [(usize, usize, u32); m],
    }

    let mut ticketab = Vec::<u32>::new();
    for ticket in xyc {
        ticketab.push(aa[ticket.0-1] + bb[ticket.1-1] - ticket.2);
    }
    ticketab.sort();

    aa.sort();
    bb.sort();
    let shopab = aa[0] + bb[0];

    println!("{}", std::cmp::min(shopab, ticketab[0]));
}
