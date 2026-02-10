

mod testcode {
    pub fn mytest() ->i32 {

        100
    }

    pub mod test2{
        pub fn ttt() ->i32 {
            200
        }
    }
}




mod testcode2 {
    pub fn mytest() ->i32 {

        100
    }

    pub mod test2{
      pub fn ttt() ->i32 {
            200
        }
    }
}





fn main() {

    println!("{}",testcode::mytest());
    println!("{}",testcode::test2::ttt());
    println!("{}",testcode2::mytest());
    println!("{}",testcode2::test2::ttt());

}
