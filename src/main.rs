fn main() {
  println!("Hello, world!!!");
  let a      = 13;
  let b: i32 = 25;
 
  let sum = return_sum(a, b);
  println!("a = {}", a);
  println!("b = {}", b);
  println!("The sum of a & b is {}", sum);
}
 
fn return_sum(i: i32, j: i32) -> i32 {
  i + j
}
