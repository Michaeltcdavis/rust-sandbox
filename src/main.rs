fn main() {
trait Harmonize {
  // If we omit the body of a method, it must be manually defined when implemented.
  fn sound(&self) -> String;
 
  // If we provide a body for a method, it serves as a default that can be overwritten.
  fn listen(&mut self) {
    std::thread::sleep_ms(3000);
  }
}
struct Human(String);
 
impl Harmonize for Human {
  fn sound(&self) -> String {
    self.0.clone()
  }
  // We do not need to implement `listen` as the default implementation is sufficient.
}
 
let mut alto = Human("oOoOo".to_string());
alto.listen();
 
let tenor = Human("ooooo".to_string());
println!("hello");
println!("{}", tenor.sound());
}
