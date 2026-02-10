
// impl Point {
//     // Self is the current datatype
//     fn new() -> Self{
//         Self {x:0,y:0}
//     }

//     //Method: Static , object method - &self is a ref to current instance 
//     fn distance_from_origin(&self) -> f32{
//         let s : i32 = self.x*self.x + self.y*self.y ;
//         (s as f32).sqrt()
//     }
//     fn distance(&self,p:&Self) -> f32{
//         let dx : i32 = self.x - p.x ;
//         let dy : i32 = self.y - p.y ;

//         let s : i32 = dx*dx + dy*dy ; 
//         (s as f32).sqrt()
//     }
// }

// #[derive(Debug)]
// struct Point{
//     x:i32,
//     y:i32
// }

// fn main(){
//     let origin : Point = Point::new();
//     let p : Point =Point{x:10,y:14};
//     let d : f32 = p.distance_from_origin();
//     // let d = Point::distance_from_origin(&p);

//     println!("{}",d);
    
//     let p2: Point = Point{x:100,y:200};
//     let d: f32 = p.distance(&p2);
//     // let d: f32 = Point::distance(&p2);
//     println!("{}",d);


//     println!("{:?}",p);
// }


// ______________________________________________________________________________________________________



// Template Programing (Meta)

#[allow(dead_code)]
#[derive(Debug)]


struct Point<T> {
    x: T,
    y:T
}



fn main(){
    let p1 : Point<i32> = Point{x:10,y:15};
    let p2 : Point<f64> = Point{x:10.5,y:3.4};

    println!("{:?}",p1);
    println!("{:?}",p2);
}
