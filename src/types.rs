/*
  Integers: i32, etc..
  Floats: f32 etc..
  Boolean (bool)
  Characters (char)
  Tuples
  Arrays
*/

pub fn run() {
    //i32 int
    let x = 1;

    //f64 int
    let f = 2.55;

    //print max size
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);

    //boolean
    let is_active = true;
    let is_greater = 10 < 5;

    //characters
    let a1 = 'a';
    let face = '\u{1F600}';

    println!("{:?}", (x, f, is_active, is_greater, a1, face));
}
