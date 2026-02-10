// fn main() {

//     //Closure (Anonymous) - Functional programming
//     let a : impl Fn(u8) -> u8 = |x: u8| x+1;
//     let b : u8 = a(10); 


// }
// ______________________________________________________________________________________________________
// fn main(){
//     let a : [i32;100] = [0;100];

//     for i in a.iter().take(10){
//         println!("{}",&i);
//     }
// }
// ______________________________________________________________________________________________________

// fn main(){
//     let a :[i32;4] = [50,30,21,70];


//     let mut sum: i32 = 0;
//     a.iter().for_each(|item| sum += item );
//     println!("{}",sum)
// }

// ______________________________________________________________________________________________________
use std::rc::Rc;

#[derive(Debug)] 
enum List {
    Pair(i32, Box<List>),
    Nil,
}


struct LinkedList{
    data:i32,
    next:Option<Box<LinkedList>>
}


struct T1 {
    v: i32,
}

struct T2 {
    d: i32,
}

trait Getable {
    fn get_value(&self) -> i32;
}

impl Getable for T1 {
    fn get_value(&self) -> i32 {
        self.v
    }
}

impl Getable for T2 {
    fn get_value(&self) -> i32 {
        self.d
    }
}

fn test(i: i32) -> Box<dyn Getable> {
    if i % 2 == 0 {
        Box::new(T1 { v: 100 })
    } else {
        Box::new(T2 { d: 200 })
    }
}

fn main() {
    let _a : LinkedList = LinkedList{data:100,next:Some(Box::new(LinkedList{data:10,next:None}))};

    let _b: List = List::Pair(100, Box::new(List::Pair(10, Box::new(List::Nil))));

    let _a: i32 = 10;
    let mut b: Box<i32> = Box::new(50);
    *b = 100;
    
    println!("{}", *b); 

    let z: Rc<i32> = Rc::new(10);
    println!("Rc value: {}", *z); 
}
