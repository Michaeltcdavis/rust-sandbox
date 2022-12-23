fn main() {
  fn less_than_5(number: i32) -> Result<bool, String> {
  if number <= 0 {
    Err("we do not want a negative number".to_string())
  } else if number < 5 {
    Ok(true)
  } else {
    Ok(false)
  }
}
    fn check_counts() -> Result<bool, String> {
  let count_a = less_than_5(-1)?;
  let count_b = less_than_5(2)?;
  let count_c = less_than_5(7)?;
 
  if count_a && count_b && count_c {
    Ok(true)
  } else {
    Ok(false)
  }
}
 
let count_err = check_counts();

if let Ok(num) = count_err {
  println!("{}", num)
} 

if let Err(val) = count_err {
  println!("{val}")
}
}
