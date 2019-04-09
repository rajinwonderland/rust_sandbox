pub fn run() {
  println!("Hello from the print rs file!");

  // Printing Integers
  println!("Number: {}", 1);

  // Basic Formatting
  println!("{} is from {}", "Raj", "is from California");

  // Positional Arguments
  println!(
    "{0} is from {1}, and {0} loves {2}!",
    "Raj", "California", "Comics"
  );

  // Named Arguments
  println!(
    "{name} likes to {verb} {noun}",
    name = "Raj",
    verb = "read",
    noun = "comics"
  );

  // Placeholder traits
  println!("Binary: {:b}, Hex: {:x}, Octal: {:o}", 10, 10, 10);

  // Placeholder for debug traits
  println!("{:?}", (12, true, "Hello"));

  // Basic Math
  println!("10 + 10 = {}", 10 + 10);
}
