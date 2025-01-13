#![allow(unused)]

fn main() {
  let x: i32 = -123; // immutable by default
  let mut y: i32 = 456; // mutable
  let z = 112;
  let a: f64  = 123.456;
  const NUM: u32 = 1;

  // x += 1; // this will fail
  y += 1;

  println!("x = {}", x);
  println!("y = {}", y);

  let x: i32 = -1;
  let x: bool = true;

  println!("x = {}", x);

  let v: Vec<_> = vec![1, 2, 3]; // _ is a type placeholder which will be replaced by the compiler
  println!("{:?}", v);
}