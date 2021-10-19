pub fn run() {
    //print to console
    println!("Hello from print.rs!");

    //formatting
    println!("{} is from {}", "Achraf", "Morocco");

    //positional arguments
    println!(
        "{0} is from {1} and {0} likes to {2}",
        "Achraf", "Morocco", "code"
    );

    //named arguments
    println!(
        "{name} likes to play {activity}",
        name = "John",
        activity = "baseball"
    );

    //placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);
}
