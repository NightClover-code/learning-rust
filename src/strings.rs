pub fn run() {
    let nickname = "Programmer";

    let mut name = String::from("Achraf");

    name.push('1');

    name.push_str(" Elmouhib");

    println!("is Empty: {}", name.is_empty());

    println!("Contains 'Achraf': {}", name.contains("Achraf"));

    println!("Replaced: {}", name.replace("Achraf", "HAHA"));

    for word in name.split_whitespace() {
        println!("{}", word);
    }

    println!("{}", name);
    println!("{}", nickname);
}
