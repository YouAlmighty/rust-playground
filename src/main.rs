fn main() {
  println!("Hello, world!!!");
  let a      = 12;
  let b: i32 = 25;
 
  let sum = return_sum(a, b);
  println!("a = {}", a);
  println!("b = {}", b);
  println!("The sum of a & b ia {}", sum);
}
 
fn return_sum(i: i32, j: i32) -> i32 {
  i + j
}

