// str -> constant -> stack segment
//&str -> because its constant , how to convert str to String ? with this -> to_owned()
// String -> Dynamic -> heap , how to convert it to str ? with this -> to_str()
// struct Info {
//     name: String,
//     family: String,
// }

// impl Showable for  Info {
//     // تابع مرتبط (Associated Function) که یک رفرنس به Info (&Info) به عنوان آرگومان می‌گیرد
//     fn show(&self) ->String {
//         format!("Data: {} - {}",self.name,self.family)
//     }
// }
// trait Showable{
//     // fn show(&self) -> String;

//     fn show(&self) -> String {
//         String::from("Not implemented!")
//     }
// }
// // Trait Bound
// fn print_data<T: Showable + Debug>(data: &T) {
// //fn print_data(data: &(impl Showable+Debug)) 
//     println!("{}",data.show());
// }


// struct IP{
//     address:String
// }
// impl Showable for  IP{
//     fn show(&self)->String{
//         format!("Data: {}",self.address)
//     }
// }

// struct Domain{
//     _name:String
// }
// impl Showable for Domain{

// }



// fn main() {
//     let m: Info = Info {
//         name: "Ali".to_owned(),
//         family: String::from("Heydari"),
//     };

//     // FIX: حذف کلمه کلیدی 'data:' از اینجا.
//     // Info::show(&m); 

//     // m.show();
//     let ip : IP = IP{address:String::from("192.168.1.1")};
//     let d : Domain  = Domain{name:"google.com".to_owned()};



//     print_data(&m);
//     print_data(&ip);
//     print_data(&d);
// }


// ______________________________________________________________________________________________________

// use std::fmt::Debug;

// // --- Trait Definition ---
// trait Showable {
//     fn show(&self) -> String{
//         String::from("Not implemented")
//     }
//     // Default implementations removed as per instruction 3, ensuring concrete types implement fully.
// }

// // --- Structs with Debug Derivation ---

// #[derive(Debug)]
// struct Info {
//     name: String,
//     family: String,
// }

// impl Showable for Info {
//     fn show(&self) -> String {
//         format!("Data: {} - {}", self.name, self.family)
//     }
// }

// #[derive(Debug)]
// struct IP {
//     address: String,
// }

// impl Showable for IP {
//     fn show(&self) -> String {
//         format!("Data: {}", self.address)
//     }
// }

// #[derive(Debug)]
// struct Domain {
//     name: String, // Corrected field name from _name to name (Instruction 4)
// }

// impl Showable for Domain {
//     fn show(&self) -> String { // Instruction 2: Completed Showable implementation for Domain
//         format!("Domain: {}", self.name)
//     }
// }

// // Trait Bound
// // Requires T to implement both Showable (for .show()) and Debug (for the bound)
// fn print_data<T: Showable + Debug>(data: &T) {
//     println!("{}", data.show());
// }


// fn main() {
//     let m = Info {
//         name: "Ali".to_owned(),
//         family: String::from("Heydari"),
//     };

//     let ip = IP {
//         address: String::from("192.168.1.1"),
//     };

//     let d = Domain {
//         name: String::from("google.com"),
//     };

//     print_data(&m);
//     print_data(&ip);
//     print_data(&d);
// }



// ______________________________________________________________________________________________________


// fn main(){
//     let mut a: String = String::from("ABCD");
//     a.push_str("EFGH");

//     let b: String = a.clone() + "IJKLMNO";

//     println!("{}",a);
//     println!("{}",b);
// }


// ______________________________________________________________________________________________________

// fn main(){

//     let mut a: Vec<i32> = vec![1,2,3,4];
//     a.push(5);


//     a[2] = 50;
//     let b:i32 = a[2];
//     let mut c: &i32 = &a[2];
//     c = &100;

//     println!("{:?}",a);
//     println!("{:?}",b);
//     println!("{:?}",c);

// }

// ______________________________________________________________________________________________________

// use std::collections::HashMap;
// use std::collections::BTreeMap;

// use std::collections::{HashMap, BTreeMap};
use std::collections::HashMap; // BTreeMap حذف شد چون استفاده نشده بود

fn main() {
    let mut a: HashMap<String, String> = HashMap::<String, String>::new();
    
    // مقدار بازگشتی b ذخیره می‌شود (اگرچه استفاده نمی‌شود)
    let b: Option<String> = a.insert("Name".to_owned(), "Javad".to_owned());

    a.insert("Name".to_owned(), "Ali".to_owned());
    a.insert("Family".to_owned(), "Heydari".to_owned());
    a.insert("Age".to_owned(), "22".to_owned());

    // println!("{}",a["Name"]);

    // اصلاح خطای E0005/E0277: حذف سینتکس k: و v:
    for (k, v) in &a {
        println!("Value for {} is {}", k, v);
    }
}
