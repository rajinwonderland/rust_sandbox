/*
@def
Primitive Types --

@desc
Integers: u8, i8, u16, i16, u32 i32, u64, i64, u128, i128 (number of bits they take up in memory)
!: u === unsigned integer (can not contain negative numbers)
!: i === integers

@desc
Float: f32, f64

@desc
Boolean: {bool}

@desc
Characters: {char}
!: fixed character, != string

@desc
Tuples
! Not covered here

@desc
Arrays
! Not covered here
*/

// @caution: Rust is a statically typed language, meaning that it must know the types of all variables at complile time! The compiler can usually infer what type we want to use based on the value and how we use it.

pub fn run() {
  // Defaul is "i32"
  let x = 1;

  // Default is "f64"
  let y = 2.5;

  // Add explicit type
  let z: i64 = 49494943030;

  // Find max size of type
  println!("Max i32: {}", std::i32::MAX);
  println!("Max i64: {}", std::i64::MAX);

  // Boolean
  let is_active: bool = true;

  // Get boolean from expression
  let is_greater = 10 > 5;

  // Characters
  //? contains on single ''
  //? contains only one
  let a1 = 'a';
  // unicode for a heart!
  let heart = '\u{2764}';

  println!("{:?}", (x, y, z, is_active, is_greater));

  println!("{}", a1);

  println!("I {} Rust!", heart);
}
