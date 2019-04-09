// Variables hold primitive data or refrences to data
// Variables are iimmutable by default
// Rust is a block supported language

pub fn run() {
  let name = "Raj";

  // Rust by default is immutable
  // mut keyword for creating mutable variable
  let mut age = 26.0;

  // as
  age = 26.5;

  println!("Me llamo {}", name);

  println!("and I'm {}", age);

  // Define Constants
  // must have type defined
  // in this case an i32 == 32 bit integer
  const ID: i32 = 001;
  println!("ID: {}", ID);

  // multiple variables
  // by default, the convention for variables should be in snake_case!
  let (superhero_name, secret_identity) = ("Spider-Man", "Peter Parker");
  println!(
    "Did you know that {}'s secret identity is {}!?",
    superhero_name, secret_identity
  );
}
