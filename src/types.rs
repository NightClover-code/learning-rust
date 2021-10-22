/*
  Integers: i32, etc..
  Floats: f32 etc..
  Boolean (bool)
  Characters (char)
  Tuples
  Arrays
*/

pub fn run() {
    let x = 1;

    let f = 2.55;

    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);

    let is_active = true;
    let is_greater = 10 < 5;

    let a1 = 'a';
    let face = '\u{1F600}';

    println!("{:?}", (x, f, is_active, is_greater, a1, face));
}
