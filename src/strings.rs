/*
Primitive str === Immutable fixed-length somewhere in memory
String === Growable; heap-allocated data structure - Use when you need to miodify, or own string data
*/

pub fn run() {
  let mut hello = String::from("Hello");

  //Get Length
  println!("length: {}", hello.len());

  // Push String
  hello.push_str(" Raj");

  println!("{}", hello);

  // Capacity in bytes
  println!("Capacity: {}", hello.capacity());

  // Check if string is empty
  println!("Is Empty: {}", hello.is_empty());

  // Contains substring
  println!("Contains 'World': {}", hello.contains("World"));

  // Replace
  println!("Replace {}", hello.replace("Raj", "Batman"));

  // Loop through string by whitespace
  for word in hello.split_whitespace() {
    println!("{}", word);
  }

  // Creates a string with capacity
  let mut s = String::with_capacity(10);
  s.push('a');
  s.push('b');

  // Assertion Testing
  // ! assertions will only print when test for assertion has failed
  assert_eq!(2, s.len());
  assert_eq!(10, s.capacity());

  println!("{}", s);
}
