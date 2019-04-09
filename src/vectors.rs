// Vectors === Resizable Arrays

// Using specific modules or packages
use std::mem;

pub fn run() {
  let mut numbers: Vec<i32> = vec![1, 2, 3, 4];

  // Re-assign value
  numbers[2] = 20;
  
  // Add on to Vector
  numbers.push(5);
  numbers.push(6);

  // Pop off last value
  numbers.pop();

    println!("{:?}", numbers);


  // get single values
  println!("Single Value: {}", numbers[0]);



  // Get Vector length
  println!("Vector Length: {}", numbers.len());

  // Vector are stack allocated
  println!("Vector occupies {} byte(s)", std::mem::size_of_val(&numbers));

  // with std::mem used!
  println!("Vector occupies {} byte(s)", mem::size_of_val(&numbers));

  // Get Sizes
  let slice: &[i32] = &numbers[0..2];
  // .. notation works like a normal ','!
  println!("Slice: {:?}", slice);

  // Loop through vector values
  for x in numbers.iter() {
    println!("Number: {}", x);
  }

  // Loop and mutate values (multiply each number by 2)
  for x in numbers.iter_mut(){
    *x *= 2;
    
  }
  println!("Numbers Vec: {:?}", numbers);
}
