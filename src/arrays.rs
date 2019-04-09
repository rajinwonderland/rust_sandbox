// Arrays === Fixed list where elements are the same data types!

// Using specific modules or packages
use std::mem;

pub fn run() {
  let mut numbers: [i32; 5] = [1, 2, 3, 4, 5];

  // Printing whole array by using the debug keyword {:?}
  println!("{:?}", numbers);

  // get single values
  println!("Single Value: {}", numbers[0]);

  // Re-assign value
  numbers[2] = 20;

  // Get array length
  println!("Array Length: {}", numbers.len());

  // Arrays are stack allocated
  println!("Array occupies {} byte(s)", std::mem::size_of_val(&numbers));

  // with std::mem used!
  println!("Array occupies {} byte(s)", mem::size_of_val(&numbers));

  // Get Sizes
  let slice: &[i32] = &numbers[0..2];
  // .. notation works like a normal ','!

  println!("Slice: {:?}", slice);
}
