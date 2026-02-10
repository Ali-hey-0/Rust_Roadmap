// fn main() {
//     let mut a: [i8; 11] = [0; 11];

//     for i in 1..=10 {
//         a[i] = i as i8;
//     }

//     println!("{:?}", &a[0..3]);
// }
// ______________________________________________________________________________________________________


// fn test(s: &String){
//     println!("{}",s.len());
// }


// fn main(){
//     let _a: i32 = 10 ; //stack 
//     let _b : Box<i32> = Box::new(20); //heap
//     let _c: [i32;10] = [5;10];
//     let _d: [i32;5] = [1,2,3,4,5];
//     // let _e: Vec<i32> = vec![1,2,3,4];
//     let f: String = String::from("hello");
//     let _g: &str = "hello";

//     test(&f);
//     println!("{}",f);
// }


// ______________________________________________________________________________________________________
// fn test_better(s_slice: &str) {
//     println!("Length: {}", s_slice.len());
// }

// fn main() {
//     let my_string: String = String::from("world");
//     let my_literal: &str = "rust";

//     test_better(&my_string); // می توانیم &String را به &str قرض دهیم
//     test_better(my_literal); // می توانیم &str را مستقیماً پاس دهیم
// }


// ______________________________________________________________________________________________________

// fn test(s: &String){
//     println!("{}",s.len());
// }

// fn add_world(s:&mut String){
//     // اصلاح: حذف پارامتر نامگذاری شده
//     s.push_str("World"); 
// }

// fn make_string() -> String{
//     // اصلاح: اطمینان از بازگشت صحیح آخرین عبارت
//     String::from("test")
// }


// fn main(){
//     let _a: i32 = 10 ; //stack 
//     let _b : Box<i32> = Box::new(20); //heap
//     let _c: [i32;10] = [5;10];
//     let _d: [i32;5] = [1,2,3,4,5];
//     // let _e: Vec<i32> = vec![1,2,3,4]; // حفظ شده با کامنت
//     let mut f: String = String::from("hello");
//     let _g: &str = "hello";

//     test(&f);
//     println!("{}",f);
//     // اصلاح: نمی‌توان &mut String را مستقیماً چاپ کرد، محتوای آن چاپ می‌شود.
//     println!("{}",f); 

//     // اصلاح: حذف پارامتر نامگذاری شده
//     f.push_str("AIon dee");

//     let ref1: &String = &f;
//     let ref2: &String = &f;

//     println!("{},{}",ref1,ref2);

//     // این خط استقراض تغییرپذیر ایجاد می‌کند و تا انتهای این بلاک معتبر است
//     let ref4: &mut String = &mut f;
//     println!("{}",ref4); 
//     // توجه: ref4 در اینجا از Scope خارج می‌شود، بنابراین f دوباره قابل دسترسی است.

//     let _data : String = make_string();

//     let a: i32 = 10;
//     let b: &i32 = &a;

// }

// ______________________________________________________________________________________________________


// fn main(){
//     let a = [1,2,3,4,5];
//     println!("{:?}",a);

//     for item in a{
//         print!("{} ",item);
//     }

//     println!("");
// }


// ______________________________________________________________________________________________________

use std::io::{self, Write, Stdin};

#[derive(Debug)]
struct Test{
    _d:i32 ,
}


fn main(){
    let a : Test = Test{_d :12};
    
    // اصلاح فرمت‌دهی: در فرمت‌دهی اعداد اعشاری، نقطه '.' باید بلافاصله بعد از ':' باشد.
    println!("Data: {:x?}",a);
    println!("Data: {:x}",10);
    println!("Data: {:X}",10);
    println!("Data: {}",10);
    println!("Data: {:.3}",3.544548454548); // اصلاح فرمت: {: .3} به {:.3} تغییر کرد
    println!("Data: #{:5}#",982);



    // اصلاح: حذف اعلان صریح نوع (i:i32) که مجاز نیست.
    for i in 1..=10 {
        for j in 1..=10{
            // اصلاح: i و j اکنون در این محدوده تعریف شده‌اند و قابل استفاده هستند.
            print!("{:4}",i * j);
        }
        println!();
    }



    println!("Data : {:>20} ***",10);
    println!("Data : {:<20} ***",10);
    println!("Data : {:^20} ***",10);
    println!("Data : {:-^20} ***",10);
    println!("Data : {:#^20} ***",10);
    println!("Data : {:0^20} ***",10);
    println!("Data : {:>^20} ***",10);




    let mut name : String = String::new();
    print!("Please enter your name: ");
    std::io::stdout().flush().unwrap();


    let stdin : Stdin = io::stdin();
    stdin.read_line(&mut name).unwrap();


    println!("Name: {}",name);


    // Caculating the ege

    let mut age : String = String::new();
    print!("Please enter your age: ");
    std::io::stdout().flush().unwrap();

    let stdin: Stdin = std::io::stdin();
    // خطا در اینجا رفع شد: (buf: &mut age) به (&mut age) تغییر کرد
    stdin.read_line(&mut age).unwrap();

    let age: u16 = age.trim().parse().unwrap();

    // خطا در اینجا رفع شد: سینتکس println! اصلاح شد
    println!("Birth: {}",2025 - age); 
}
