//

// ______________________________________________________________________________________________________

// use std::thread;
// use std::thread::JoinHandle; // Importing the correct type

// #[derive(Debug)]
// enum MakeThreadError {
//     EJoin,
//     EFail,
//     EUnknown
// }

// // Adjusted signature to be slightly cleaner, though your original was valid for panics
// fn make_thread() -> Result<(), Box<dyn std::any::Any + Send>> {

//     // CORRECTION 1: Type is JoinHandle, not JoinHandler
//     let handle: JoinHandle<()> = thread::spawn(|| {
//         println!("Hello from the thread!");
//     });

//     // --- Error Recovery Concepts ---

//     // Option A: Using Match (Verbose)
//     /*
//     match handle.join() {
//         Ok(_) => println!("Thread finished successfully"),
//         Err(e) => println!("Thread panicked: {:?}", e),
//     }
//     */
//     // Option B: Using if let (Concise check)
//     // Note: It is .join(), not .json()
//     /*
//     let res = handle.join();
//     if res.is_err() {
//         println!("Error in thread");
//         // We can't return your custom Enum here easily because
//         // the function signature expects Box<dyn Any + Send>
//         // return Err(Box::new("Custom error string"));
//     }
//     */
//     // CORRECTION 2: Actual execution
//     // The '?' operator will propagate the error if the CHILD thread PANICS.
//     handle.join()?;

//     // CORRECTION 3: You must return Ok at the end
//     Ok(())
// }

// fn main() {
//     // We handle the result here. unwrap() will crash main if make_thread fails.
//     make_thread().unwrap();
// }

// ______________________________________________________________________________________________________

// use std::thread;

// fn make_thread() -> Result<(), Box<dyn std::any::Any + Send>> {
//     let handle = thread::spawn(|| {
//         println!("Hello!");
//     });

//     handle.join()?;
//     Ok(())
// }

// fn main() {
//     make_thread().unwrap();
// }

// ______________________________________________________________________________________________________

// use std::thread;
// use crate::thread::JoinHandle

// fn main(){
//     let data: Vec<i32> = (1..101).collect();

//     let mut threads : Vec<JoinHandle<i32>> = Vec::new();

//     for _ in 0..2{
//         threads.push(thread::spawn(move || {
//             let mut res: i32 = 0;
//             data.iter().for_each(|&item| res += item);
//             res
//         }));
//     }
//     // .join();
//     // unwrap();

//     let mut result: i32 = 0;
//     for t in threads{
//         result += t.join().unwrap();
//     }

//     println!("Result: {:?}", result);
// }

// ______________________________________________________________________________________________________
// use std::thread;
// use std::sync::Arc;
// use std::thread::JoinHandle;

// fn main() {
//     // Create data
//     let data: Vec<i32> = (1..=100).collect();

//     // Wrap data in Arc so it can be shared across threads
//     let data_ptr: Arc<Vec<i32>> = Arc::new(data);

//     let mut threads: Vec<JoinHandle<i32>> = Vec::new();
//     let thresh: usize = 50;

//     for i in 0..2 {
//         let td = Arc::clone(&data_ptr);

//         threads.push(thread::spawn(move || -> i32 {
//             let start = i * thresh;
//             let sum: i32 = td.iter()
//                 .skip(start)
//                 .take(thresh)
//                 .sum();

//             sum
//         }));
//     }

//     // Join threads and collect results
//     let total: i32 = threads
//         .into_iter()
//         .map(|t| t.join().unwrap())
//         .sum();

//     println!("Total sum: {}", total);
// }
