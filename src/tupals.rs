// Tupals group together values of different types
// @warning Max 12 elements!

pub fn run() {
  let person: (&str, &str, i8) = ("Raj", "Singh", 26);

  println!(
    "{}'s last name is {} and is a youthful {} years of age",
    person.0, person.1, person.2
  );
}
