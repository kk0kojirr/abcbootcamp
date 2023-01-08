fn main() {
    let test_string = String::from("this is format of String");
    println!("{}", test_string);

    let mut test_concat: String = test_string + ":test" + ":test";
    println!("{}", test_concat);

    let more_string: String = String::from(":more string");
    let more_stringa: String = String::from(":more new string");

    test_concat = test_concat + &more_string + &more_stringa;
    println!("{}", test_concat);

    let stringtest = String::from("🍣好き💖");
    // 部分文字列
    let mut straa = "".to_string();
    for (i, c) in stringtest.chars().enumerate() {
        if i >= 1 && i < 3 {
            straa.push(c);
        }
    }
    println!("{}", straa);

    // 部分文字列
    let sub = stringtest.chars().skip(1).take(2).collect::<String>();
    println!("{}", sub);
}
