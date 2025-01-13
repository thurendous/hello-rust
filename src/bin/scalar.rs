#![allow(unused)]
// #![allow(arithmetic_overflow)]

fn main() {
  // Scaler types are the most basic types in Rust
  // - single value
  // - building blocks for more complex types
  // Integers
  // Signed integers: i8, i16, i32, i64, i128
  let i0: i8 = 127;
  let i1: i16 = 1;
  let i2: i32 = 1;
  let i3: i64 = 1;
  let i4: i128 = 1;
  let i5: isize = 1; // depends on the architecture of the machine
  // Unsigned integers: u8, u16, u32, u64, u128
  // from 0 ~ 2**n - 1
  let u0: u8 = 1; // 0 ~ 255
  let u1: u16 = 1; // 0 ~ 65535
  let u2: u32 = 1; // 0 ~ 4294967295
  let u3: u64 = 1; // 0 ~ 18446744073709551615
  let u4: u128 = 1; // 0 ~ 340282366920938463463374607431768211455
  let u5: usize = 1; // depends on the architecture of the machine
  // Floating point numbers: f32, f64
  let f0: f32 = 1.01; // 32-bit floating point number
  let f1: f64 = 1.01; // 64-bit floating point number
  // Boolean: bool
  let b0: bool = true; // true or false
  let b1: bool = false;
  // Characters: char
  let c0: char = 'a'; // single quote means this is a character, and a double quote means this is a string literal
  let e: char = '😊'; // emoji

  // Type conversion
  let i: i32 = 123;
  let u: u32 = i as u32;
  let x: u32 = u + (i as u32);

  println!("{}", x);

  // MIN and MAX
  let min_i: i32 = i32::MIN;
  let max_i: i32 = i32::MAX;

  println!("i32 min: {min_i}");
  println!("i32 max: {max_i}");

  // char min and max
  let min_char: char = '\0';         // Unicode 0x0000
  let max_char: char = '\u{10FFFF}'; // Maximum Unicode scalar value

  println!("char min: {min_char}");
  println!("char max: {max_char}");

  // overflow
  let mut u: u32 = u32::MAX;
  u += 1;

  println!("Overflow: u32 max + 1: {x}");

  // checked_add
  let u: u32 = u32::MAX;
  let x: Option<u32> = u.checked_add(1);
  println!("Checked add: {x:?}");

  // wrapping_add
  let u: u32 = u32::MAX;
  let x: u32 = u.wrapping_add(1);
  println!("Wrapping add: {x}");
}



