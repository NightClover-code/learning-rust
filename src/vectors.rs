pub fn run() {
    let mut nums: Vec<i32> = vec![2, 6, 4, 6, 5];

    nums.push(8);

    nums.pop();

    for x in nums.iter() {
        println!("Num: {}", x);
    }

    for x in nums.iter_mut() {
        *x *= 2;
    }

    println!("Nums Vector: {:?}", nums);
}
