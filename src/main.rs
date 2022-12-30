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
println!("{}", tenor.sound());
println!("-------------------------------------");

struct Passerine {
  freq: Vec<u64>,
}
 
impl Harmonize for Passerine {
  fn sound(&self) -> String {
    format!("{:?}", self.freq)
  }
  fn listen(&mut self) {
    self.freq.push(432);
    std::thread::sleep_ms(303);
  }
}
 
let mut bird = Passerine {
  freq: vec![28, 37, 108, 92],
};

bird.listen();
println!("{}", bird.sound());

macro_rules! make_it {
  ( $var:ident => $($count:expr),+) => {
    $($var.push($count);)+
  }
}
#[allow(unused_variables)]
  let variable = "unused";
  
let mut count = vec![];
 
make_it![count => u8::MIN, 1, 2];
 
println!("{count:?}");
}
