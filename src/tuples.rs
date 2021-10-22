pub fn run() {
    let person: (&str, &str, i8) = ("Achraf", "Morocco", 37);

    println!(
        "{} is from {} and is {} years old.",
        person.0, person.1, person.2
    );
}
