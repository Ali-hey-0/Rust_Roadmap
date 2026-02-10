//printlines!("Hi" , 10 , false);

//println!("{}", "HI");
//println!("{}", 10);
//println!("{}", false);

#[macro_export]
macro_rules! printlines {
    ($($line:expr),*) => {
        { $(
            println!("Value is:");
            println!("{}", $line);
            println!();
        )*}

    };
}
