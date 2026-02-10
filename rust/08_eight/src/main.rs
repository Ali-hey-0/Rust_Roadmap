use std::rc::Rc;


#[derive(Debug)]
enum List {
    Pair(i32,Box<List>),
    Nil
}

use crate::List::{Nil,Pair};

struct Test {
    _V:i32
}


fn main() {
    // let data : List = List::Pair(10,Box::new(List::Nil));
    let data : List = Pair(10,Box::new(List::Nil));


    // let b:Box<List> = Box::new(data);  ---- value moved here
    // let c:Box<List> = Box::new(data); ^^^^ value used here after move

    let b = Rc::new(data);
    let c = Rc::clone(&b);
    let d = Rc::clone(&b);



    println!("{:?}",b);
    println!("{:?}",c);
    println!("{:?}",d);
}
