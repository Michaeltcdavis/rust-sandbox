fn main() {
    let array = [1,2,3];
    let array2 = ['c'; 3];
    for n in array {
      println!("{n}")
    }
    array2.iter().map(|c| println!("{c}"));

    let a = "testing";
    let b = " some";
    let c = " strings";
    println!("{a}{b}{c}!")
}
