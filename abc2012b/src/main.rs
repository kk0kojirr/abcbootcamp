fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).ok();
    let mut prev: bool = false;
    for c in buf.chars() {
        if c == ' ' && !prev {
            print!(",");
            prev = true;
        } else if c != ' ' {
            print!("{}", c);
            prev = false;
        } else {
            //
        }
    }
}
