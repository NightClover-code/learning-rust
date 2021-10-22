pub fn run() {
    let mut nums: [i32; 5] = [2, 6, 4, 6, 5];

    nums[2] = 20;

    println!("Nums Array: {:?}", nums);

    println!("Single Value: {}", nums[0]);

    println!("Array length: {}", nums.len())
}
