#![allow(unused)]

fn main() {
  // Tuple
  let t: (bool, u32, char) = (true, 10, 'a');
  println!("1,{}", t.0);
  println!("2,{}", t.1);
  println!("3,{}", t.2);

  // destructuring
  let (b, u, c) = t;
  // ignore with _
  let (_, w, _) = t;
  println!("4,{}", b);
  println!("5,{}", u);
  println!("6,{}", c);
  println!("7,{}", w);

  // array
  let a: [i32; 3] = [1, 2, 3];
  println!("8,{}", a[0]);
  println!("9,{}", a[1]);
  println!("10,{}", a[2]);

  // empty tuple
  let e = ();

  // nested tuple
  let nested = ((1.23, 'a'), (true, 1u32, 'b'));
  println!("11, {} {} {}", nested.0.0, nested.0.1, nested.1.1);

  // Array - fixed length, known at compile time
  let arr = [1, 2, 3];
  println!("12, {} {} {}", arr[0], arr[1], arr[2]);

  let mut arr2 = [1, 2, 3];
  arr2[2] = 4;
  arr2[1] = 5;
  arr2[0] = 6;
  println!("13, {} {} {}", arr2[0], arr2[1], arr2[2]);

  let mut arr3: [i32; 10] = [0; 10]; // is the same as `let mut arr3 = [0,0,0,0,0,0,0,0,0,0];`
  arr3[0] = 1;
  arr3[1] = 2;
  arr3[2] = 3;
  println!("14, {} {} {}", arr3[0], arr3[1], arr3[2]);

  // Slice - length not known at compile time
  let num_list: [i32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
  let slice = &num_list[1..5]; // starting from the index 1, up to the index 5 (not included)
  // &num_list[..2] is the same as &num_list[0..2], `0` is optional
  // &num_list[2..] is the same as &num_list[2..10], `10` is optional
  // [2, 3, 4, 5]
  println!("15, {:?}", slice);

  // all elements
  let all_elements = &num_list[..];
  println!("16, {:?}", all_elements);

}
