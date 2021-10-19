pub fn run() {
    //mutable / immutable vars
    let name = "Achraf";
    let mut age = 54;

    println!("My name is {} and I'm {}.", name, age);
    age = 65;
    println!("My name is {} and I'm {}.", name, age);

    //constants
    const ID: i32 = 001;
    println!("ID: {}", ID);

    //multiple vars
    let (my_name, my_age) = ("Achraf", 20);

    println!("My name is {} and I'm {}.", my_name, my_age);
}
