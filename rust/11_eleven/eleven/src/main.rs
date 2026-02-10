

// crate:: point to current crate


mod mycode;

use mycode::mytestcode;


fn main() {
    let a: i32 = mytestcode();
    println!("{}",a);
}
