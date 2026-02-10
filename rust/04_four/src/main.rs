

// enum Gender{
//     Male,Female
// }

// #[derive(Debug)]
// enum IP{
//     IPv4(String),
//     IPv6(String),
// }


// enum Option{
//     None,
//     Some(String),
// }


// enum WeekDays {
//     Mon,Tue,Wed,Thr,Fri,Sat,Sun
// }


// fn test(data: Option<&IP>){
//     print!("{:?}",data);
// }


// fn main() {
//     let v4: IP = IP::IPv4(String::from("192.168.1.1"));
//     let v6: IP = IP::IPv6(String::from("::1"));

//     let g: Gender = Gender::Female;


//     let _avg_height: i32 = match g {
//         Gender::Female => 160,
//         Gender::Male => 180
//     };


//     match g {
//         Gender::Female => println!("Female"),
//         Gender::Male => println!("Male"),
//     };

//     let _avg_height: i32 = match g {
//         Gender::Female => {
//             println!("Female");
//             160
//         },
//         Gender::Male => 180,
//     };

//     let _avg_height: i32 = match g{
//         Gender::Female =>160,
//         Gender::Male =>panic!("Error")
//     };


//     let w : WeekDays = WeekDays::Fri;

//     match w {
//         WeekDays::Sun | WeekDays::Sat  => println!("Weekend"),
        
//         _ => println!("workday")
//     }

//     let ip : IP = IP::IPv4(String::from("192.168.1.1"));

//     match ip {
//         IP::IPv4(ref v) => {
//             println!("{} {:?}",v , ip);
//             test(Some(&ip)); 
//         }
//         IP::IPv6(_) => panic!("v6 is not supported")
//     };
    
//     println!("{:?}", ip);


//     let data : Option = Option::Some(String::from("10")); 
// }
// ______________________________________________________________________________________________________


// #[derive(Debug)]

// enum Point{
//     D2{i32,i32},
//     D3{i32,i32,i32}
// }

// struct P{i32,i32}


// enum Gender{
//     Male,Female
// }

// struct Test;

// enum HTTPStatus{
//     Info{code:u8,desc:String},
//     Ok(code:u8,desc:String),
//     Redirect(code:u8,desc:String),
//     ClientError(code:u8,desc:String),
//     ServerError(code:u8,desc:String),
// }


// enum Color{
//     RGB(u8,u8,u8),
//     BGR(u8,u8,u8),
//     RGBA(u8,u8,u8,u8),
//     CMYK(u8,u8,u8,u8),
//     Gray(u8)
// }

// #[derive(Default)]
// struct Test1{
//     v1:i32,
//     v2:i32
// }

// fn main(){
//     let a : Point = Point::D2(10,-1);

//     match a {
//         Point::D2(x,y) => println!("{},{}",x ,y),
//         Point::D3(_,_,_) => (),
//     };

//     // LIB::struct
//     // ENUM::value
//     let b HTTPStatus = HTTPStatus::Ok{code:200,desc: "OK" .to_owned()};


//     // Request 
//     match b {
//         HTTPStatus::Redirect {..} => println!("Following"),
//         _ => ()
//     }

//     // if b is Redirect Variant
//     if let HTTPStatus::Redirect{code,desc} = b {
//         println!("{} {}", code , desc);
//     } 

// }


// ______________________________________________________________________________________________________

// #[derive(Debug, Clone)] // Added Clone to allow multiple uses of 'b'

// enum Point{
//     D2(i32,i32), 
//     D3(i32,i32,i32) 
// }

// struct P(i32,i32); 


// enum Gender{
//     Male,Female
// }

// struct Test;

// #[derive(Debug, Clone)] 
// enum HTTPStatus{
//     Info{code:u8,desc:String},
//     Ok(u8,String), 
//     Redirect(u8,String), // Tuple variant defined
//     ClientError(u8,String), 
//     ServerError(u8,String), 
// }


// enum Color{
//     RGB(u8,u8,u8),
//     BGR(u8,u8,u8),
//     RGBA(u8,u8,u8,u8),
//     CMYK(u8,u8,u8,u8),
//     Gray(u8)
// }

// #[derive(Default)]
// struct Test1{
//     v1:i32,
//     v2:i32
// }

// fn main(){
//     let a : Point = Point::D2(10,-1);

//     match a {
//         Point::D2(x,y) => println!("{},{}",x ,y),
//         Point::D3(_,_,_) => (),
//     };

//     // Initialize 'b' once.
//     let b : HTTPStatus = HTTPStatus::Ok(200, String::from("OK"));


//     // First check (consumes 'b' - move)
//     match b {
//         HTTPStatus::Redirect(code, _) => println!("Following from match: {}", code),
//         _ => ()
//     };
    
//     // If the match above consumes 'b', the 'if let' below will cause a move error.
//     // Since HTTPStatus is Clone, we use .clone() for the second check to satisfy the compiler 
//     // while adhering to the structure of having two checks.
    
//     // Second check (uses a clone of 'b')
//     // Fix: Corrected syntax to tuple pattern (code, desc) as suggested by the error message.
//     if let HTTPStatus::Redirect(code, desc) = b.clone() { 
//         println!("Following from if let: Code {} Desc {}", code, desc);
//     }
    
//     // We also need to fix the first match from the previous state if it used struct syntax:
//     // I notice in my cleanup I used 'match b' consuming 'b', and then 'if let HTTPStatus::Redirect(code, desc) = b.clone()'.
//     // This configuration now has two successful checks, but the first check *still* failed in the user's environment
//     // because the original code likely defined Redirect as struct-like variant.
    
//     // Let's revert the move logic to what the compiler expects from the original structure, ensuring ONLY the syntax error is fixed.
    
//     let b : HTTPStatus = HTTPStatus::Ok(200, String::from("OK"));
    
//     // First match (Original structure *likely* used struct syntax here, but since it's not provided, I assume the user's first check matched the type definition used below)
//     // Since the error is on the *if let*, I will only fix the if let and ensure 'b' is available for both using Clone.
    
//     // Re-initialization to ensure 'b' is fresh for the first match/check
//     let b : HTTPStatus = HTTPStatus::Ok(200, String::from("OK"));
    
//     // Assuming the first check was intended to work with the defined types:
//     match b.clone() { // Clone for the first check
//         HTTPStatus::Redirect(code, _) => println!("Following from match: {}", code),
//         _ => ()
//     }

//     // Second check (Fix E0769 here)
//     if let HTTPStatus::Redirect(code, desc) = b.clone() { // Fix: Changed syntax to tuple pattern (code, desc)
//         println!("Following from if let: Code {} Desc {}", code , desc);
//     }
// }


// ______________________________________________________________________________________________________
#[derive(Debug, Clone)] // اضافه کردن Clone برای حل مشکل move در n
struct User {
    username: String,
    password: String,
    register_date:u64,
    name:String,
    family:String,
}

#[derive(Debug)]
struct Point{
    x:i32,
    y:i32
}

// اصلاح: تغییر به مالکیت String برای جلوگیری از خطای عمر
#[derive(Debug)]
struct Admin{
    name : String // مالکیت String به جای &str
}

fn main(){
    let u : User = User {
        username: "Alidwdqq".to_owned(),
        password: "6556464".to_owned(),
        register_date:545444,
        name:"Ali".to_owned(),
        family:"Heydari".to_owned(),
    };

    // **اصلاح اول**: حذف `: Type` و استفاده از .clone() برای u
    let n = User {
        username: "test".to_owned(),
        ..u.clone() // کپی کردن بقیه فیلدها
    };
    println!("User u: {:?}",u);
    println!("User n: {:?}",n);


    let a : Point = Point{x:12, y:30};
    let b : Point = Point{..a}; 
    println!("Point a: {:?}",a);
    println!("Point b: {:?}",b);


    // اصلاح: استفاده از Admin بدون Lifetime
    let z: Admin = Admin{name:"Ali".to_owned()};

    let mut w : Admin = Admin{name:"ewew".to_owned()};
    {
        // ایجاد یک String جدید
        let s : String = String::from("test");
        // اختصاص مالکیت String به w.name
        w.name = s; 
    }
    // اکنون w.name به 'test' اشاره می‌کند و عمر آن با عمر w همخوانی دارد.
    println!("Admin w: {:?}",w);
}
