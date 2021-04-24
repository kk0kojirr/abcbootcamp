use proconio::input;
fn main() {
    input! {
        s: String,
    }
    match &*s {
        "Sunny" => println!("Cloudy"),
        "Cloudy" => println!("Rainy"),
        "Rainy" => println!("Sunny"),
        _ => println!(""),
    };
}
