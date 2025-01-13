#![allow(unused_variables)]

// String and &str
// str is called a "string slice"
// String is a valid UTF-8 sequence of bytes

fn main() {
  // String = vector of u8 (Vec<u8>) valid UTF-8 sequence of bytes
  // &str = slice of u8 (&[u8]) valid UTF-8

  // When to use String and &str?
  // String -> mutate or data needs to be owned
  // &str -> read only

  // String 
  let msg: String = String::from("Hello");
  let len = msg.len();
  println!("1, len {}", len);

  // &str
  // - usually used str with reference
  // - immutable
  let msg: String = String::from("Hello");
  let s: &str = &msg[0..5];
  println!("2, {}", s);
  let len: usize = s.len();
  println!("3, len {}", len);

  // String literal
  // - stored inside the binary
  // - slice pointing to a specific part of the binary
  // - immutable because hard-coded inside the binary
  let hello: &str = "Hello";

  let s: &str = r#"
  {"a": 1,
   "b": 2,
   "c": 3
  }
  Hello
  World
  Hello
  Now
  "#;
  println!("4, {}", s);

  // Add &str to String
  let mut msg: String = "Hello Rust".to_string();
  msg += "!!";
  println!("5, {}", msg);

  let lang = "Rust";
  let emoji = "🦀";
  let msg = format!("Hello {lang} {emoji}");
  println!("6, {}", msg);
}
