pub fn run() {
    let name = "Achraf";
    let mut age = 54;

    println!("My name is {} and I'm {}.", name, age);
    age = 65;
    println!("My name is {} and I'm {}.", name, age);

    const ID: i32 = 001;
    println!("ID: {}", ID);

    let (my_name, my_age) = ("Achraf", 20);

    println!("My name is {} and I'm {}.", my_name, my_age);
}
