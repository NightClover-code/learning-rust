pub fn run() {
    println!("Hello from print.rs!");

    println!("{} is from {}", "Achraf", "Morocco");

    println!(
        "{0} is from {1} and {0} likes to {2}",
        "Achraf", "Morocco", "code"
    );

    println!(
        "{name} likes to play {activity}",
        name = "John",
        activity = "baseball"
    );

    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);
}
