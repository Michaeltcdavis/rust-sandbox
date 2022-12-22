fn main() {
    let numbers = [1, 2, 3];
 
let nums: Vec<i32> = numbers.iter()
  .map(|x| x * 2 )
  .collect();
 
println!("{nums:?}"); // [2, 4, 6]
}
