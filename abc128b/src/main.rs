use proconio::input;

fn main() {
    input! {
        n: u32,
    }

    let mut table: Vec<(String, i32, u32)> = vec![];

    for i in 0..n {
        input! {
            name: String,
            p: i32,
        }
        let input: (String, i32, u32) = (name, -1 * p, i);
        table.push(input);
    }
    
    table.sort();

    for (_, _, i) in table {
        println!("{}", i + 1);
    }
}
