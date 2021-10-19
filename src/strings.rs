pub fn run() {
    //immut string
    let nickname = "Programmer";

    //mutable string
    let mut name = String::from("Achraf");

    //push char
    name.push('1');

    //push string
    name.push_str(" Elmouhib");

    //check if empty
    println!("is Empty: {}", name.is_empty());

    //contains
    println!("Contains 'Achraf': {}", name.contains("Achraf"));

    //replace (not muting original)
    println!("Replaced: {}", name.replace("Achraf", "HAHA"));

    //loop through strings
    for word in name.split_whitespace() {
        println!("{}", word);
    }

    //print names
    println!("{}", name);
    println!("{}", nickname);
}
