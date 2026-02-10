//Promise , Future , async/await , Task , Executor , Reactor , non-blocking IO

use futures::executor::block_on;

async fn test() {
    async_std::task::sleep(std::time::Duration::from_secs(5)).await;
    println!("Async test function");
}

async fn entry() {
    // test().await;
    let f1 = test();
    let f2 = test();
    futures::join!(f1, f2);
}

fn main() {
    // let future = test();
    // block_on(future);

    block_on(entry());
}
