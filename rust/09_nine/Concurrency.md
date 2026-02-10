
# 🦀 Rust Concurrency: A Comprehensive Guide

Concurrency is one of Rust's boldest features. Rust guarantees **fearless concurrency**, meaning it allows you to write concurrent code that is free of data races by design. The compiler catches thread-safety errors at compile time rather than runtime.

## 1. Threads: The Basics

Rust uses a **1:1 threading model**, meaning one Rust thread corresponds to one operating system thread.

### Creating a New Thread

To create a thread, we use `thread::spawn`.

```rust
use std::thread;
use std::time::Duration;

fn main() {
    // Spawn a new thread
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    // Main thread code
    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    // Wait for the spawned thread to finish
    handle.join().unwrap();
}
```

### Using `move` Closures

To use data from the main thread inside the spawned thread, the closure must take ownership of the values using the `move` keyword.

```rust
use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}
```

---

## 2. Message Passing

Rust follows the philosophy: *"Do not communicate by sharing memory; instead, share memory by communicating."* This is achieved using **channels**.

### `mpsc` Channels

Rust's standard library provides `mpsc` (multiple producer, single consumer).

```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    // Create a channel
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap(); // Send value to the receiver
        // println!("val is {}", val); // Error! val is moved and no longer valid here
    });

    let received = rx.recv().unwrap(); // Block until a message is received
    println!("Got: {}", received);
}
```

| Method         | Description                                                                          |
| :------------- | :----------------------------------------------------------------------------------- |
| `send()`     | Sends a value. Takes ownership. Returns a `Result`.                                |
| `recv()`     | Blocks the main thread's execution and waits until a value is sent down the channel. |
| `try_recv()` | Does not block. Returns `Result<T, TryRecvError>` immediately.                     |

---

## 3. Shared-State Concurrency

Sometimes you do need to share memory. Rust handles this safely using Mutexes and Atomic Reference Counting.

### Mutex (`Mutex<T>`)

A **Mutex** (mutual exclusion) allows only one thread to access some data at any given time. To access the data, a thread must signal that it wants access by asking for the `lock`.

```rust
use std::sync::Mutex;

fn main() {
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    } // Lock is automatically released (dropped) here

    println!("m = {:?}", m);
}
```

### `Arc<T>`: Atomic Reference Counting

Because `Mutex<T>` is not `Copy`, you cannot move it into multiple threads. `Rc<T>` is not thread-safe. Therefore, we use `Arc<T>` (Atomic Reference Counted) to share ownership across threads.

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // Wrap the Mutex in an Arc to share ownership
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
```

---

## 4. `Send` and `Sync` Traits

These two marker traits are deeply embedded in the language and ensure concurrency safety.

### The `Send` Trait

The `Send` trait indicates that ownership of values of the type implementing `Send` can be transferred between threads.

* Almost every Rust type is `Send`.
* Exceptions include `Rc<T>` (not thread-safe) and raw pointers.

### The `Sync` Trait

The `Sync` trait indicates that it is safe for the type implementing `Sync` to be referenced from multiple threads.

* In other words, type `T` is `Sync` if and only if `&T` (an immutable reference to `T`) is `Send`.
* `Mutex<T>` is `Sync`, which allows it to be shared. `RefCell<T>` is not.

---

## 5. Summary Comparison

| Concept                    | Tool / Type            | Use Case                                         |
| :------------------------- | :--------------------- | :----------------------------------------------- |
| **Execution**        | `std::thread::spawn` | Running code in parallel.                        |
| **Communication**    | `std::sync::mpsc`    | Sending data between threads (Stream of data).   |
| **State Sharing**    | `Mutex<T>`           | Single access to data.                           |
| **Shared Ownership** | `Arc<T>`             | Letting multiple threads own the same lock/data. |

---



That is a great initiative. To truly master Rust concurrency, we need to look beyond the basic `<span data-markdown-start-index="96">spawn</span>` and `<span data-markdown-start-index="108">Mutex</span>`. We need to understand **optimization** (Read/Write locks), **lifetimes in threads** (Scoped threads), **lock-free programming** (Atomics), and **coordination** (Barriers).

Here is a deeper dive into advanced Rust concurrency concepts.

---

# ⚓ Advanced Rust Concurrency: Deep Dive

## 1. `<span data-markdown-start-index="403">RwLock<T></span>` (Read-Write Lock)

A `<span data-markdown-start-index="434">Mutex<T></span>` is simple but aggressive: it blocks **everyone** else, whether they want to read or write.
A `<span data-markdown-start-index="537">RwLock<T></span>` is smarter. It allows **multiple readers** at the same time, but only  **one writer** .

* **Use Case:** Systems where data is read often but updated rarely (e.g., a configuration object).
* **Rules:**
  * Many readers can hold the lock simultaneously.
  * If a writer holds the lock, no one else (reader or writer) can access it.

```rust
use std::sync::{Arc, RwLock};
use std::thread;

fn main() {
    let lock = Arc::new(RwLock::new(5));
    let c_lock = Arc::clone(&lock);

    // Reader Thread (allows multiple of these at once)
    let r1 = thread::spawn(move || {
        let r = c_lock.read().unwrap(); // Request Read access
        println!("Reader 1: {}", *r);
    });

    // Writer Thread (blocks everyone else)
    let w1 = thread::spawn(move || {
        let mut w = lock.write().unwrap(); // Request Write access
        *w += 1;
        println!("Writer updated value to: {}", *w);
    });

    r1.join().unwrap();
    w1.join().unwrap();
}
```

---

## 2. Scoped Threads (`<span data-markdown-start-index="1522">thread::scope</span>`)

In the basic examples, we often used `<span data-markdown-start-index="1575">Arc</span>` or `<span data-markdown-start-index="1584">move</span>` because standard threads are `<span data-markdown-start-index="1620">'static</span>`—the compiler assumes they might run forever, so they can't borrow local variables that might be dropped.

**Scoped Threads** (introduced in Rust 1.63) guarantee that the threads will finish *before* the current function scope ends. This allows you to borrow local variables **without** `<span data-markdown-start-index="1915">Arc</span>`.

```rust
use std::thread;

fn main(){
    let mut vec = vec![1, 2, 3];

    // Create a scope. All threads spawned here MUST finish before the scope closes.
    thread::scope(|s| {
        // Spawn a thread that borrows 'vec' immutably
        s.spawn(|| {
            println!("Length of vec: {}", vec.len());
        });

        // Spawn another thread that borrows 'vec' immutably
        s.spawn(|| {
            println!("First element: {}", vec[0]);
        });
    }); 
    // <--- The code effectively pauses here until all inner threads are done.

    println!("All threads finished, vec is still here: {:?}", vec);
}
```

**Why this matters:** It removes the overhead of `<span data-markdown-start-index="2584">Arc</span>` and makes code much cleaner when you just want to parallelize a task locally.

---

## 3. Atomics (`<span data-markdown-start-index="2687">std::sync::atomic</span>`)

For simple primitive types (integers, booleans), `<span data-markdown-start-index="2756">Mutex</span>` is overkill. A Mutex involves asking the Operating System to put a thread to sleep, which is "expensive" (slow).

**Atomics** use direct CPU instructions to manage memory safely without locking. They are incredibly fast but harder to use correctly.

```rust
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;

fn main(){
    // AtomicUsize is a thread-safe unsigned integer
    let counter = Arc::new(AtomicUsize::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let c = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            // fetch_add atomically adds 1. No Mutex needed!
            c.fetch_add(1, Ordering::Relaxed);
        }));
    }

    for h in handles { h.join().unwrap(); }

    println!("Count: {}", counter.load(Ordering::Relaxed));
}
```

* **Ordering (`<span data-markdown-start-index="3591">Relaxed</span>`, `<span data-markdown-start-index="3602">SeqCst</span>`, etc.):** This tells the compiler how strictly it needs to order memory operations. `<span data-markdown-start-index="3695">Relaxed</span>` is the fastest but provides fewer guarantees about when other threads see the change. `<span data-markdown-start-index="3791">SeqCst</span>` (Sequentially Consistent) is the strict default.

---

## 4. Thread Coordination: `<span data-markdown-start-index="3880">Barrier</span>`

Sometimes you want to spawn 10 threads, but you don't want *any* of them to proceed to step 2 until *all* of them have finished step 1.

A `<span data-markdown-start-index="4027">Barrier</span>` makes threads wait until a specific number of threads have arrived at the barrier.

```rust
use std::sync::{Arc, Barrier};
use std::thread;

fn main(){
    let n_threads = 3;
    // Create a barrier that blocks until 3 threads call wait()
    let barrier = Arc::new(Barrier::new(n_threads)); 

    let mut handles = vec![];

    for i in 0..n_threads {
        let c_barrier = Arc::clone(&barrier);
        handles.push(thread::spawn(move || {
            println!("Thread {} is preparing...", i);
            // Do some work...
          
            // WAIT here until all 3 threads reach this line
            c_barrier.wait(); 
          
            println!("Thread {} is continuing!", i);
        }));
    }

    for h in handles { h.join().unwrap(); }
}
```

---

## 5. The Dangers: Deadlocks & Poisoning

### Deadlocks

A deadlock happens when Thread A holds Lock 1 and waits for Lock 2, while Thread B holds Lock 2 and waits for Lock 1. Neither can finish. Rust **does not** prevent deadlocks at compile time; you must design your logic to avoid them (usually by always acquiring locks in the same order).

### Mutex Poisoning

If a thread holds a `<span data-markdown-start-index="5169">Mutex</span>` and then **panics** before releasing it, the lock becomes "poisoned." Rust assumes the data inside might be corrupted (half-written).

This is why `<span data-markdown-start-index="5323">lock()</span>` returns a `<span data-markdown-start-index="5342">Result</span>`.

* `<span data-markdown-start-index="5355">.unwrap()</span>`: "If the lock is poisoned, panic this thread too." (Most common strategy).
* Handle the error: You can technically retrieve the data even if poisoned, but it's risky.

```rust
let mut guard = match lock.lock() {
    Ok(guard) => guard,
    Err(poisoned) => poisoned.into_inner(), // Recover data despite panic
};
```

---

## Summary of Advanced Tools

| Tool                                                                      | Best For...                                                                                             |
| ------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------- |
| **`<span data-markdown-start-index="5753">RwLock<T></span>`**     | High read frequency, low write frequency.                                                               |
| **`<span data-markdown-start-index="5816">thread::scope</span>`** | Parallelizing work on local variables (no `<span data-markdown-start-index="5878">Arc</span>`needed). |
| **`<span data-markdown-start-index="5898">Atomic</span>`types**   | High-performance counters or status flags (no locking).                                                 |
| **`<span data-markdown-start-index="5978">Barrier</span>`**       | Synchronizing multiple threads at a specific checkpoint.                                                |
| **`<span data-markdown-start-index="6054">Condvar</span>`**       | Threads that need to sleep until a specific condition is met (e.g., "queue is not empty").              |




This is where Rust gets really interesting.

To understand these concepts, we have to switch our mindset from **OS Threads** (which we discussed before) to  **Asynchronous Programming** .

Here is the breakdown of Async, Futures, Goroutines, and Coroutines, explained simply and how they fit into Rust.

---

# ⏳ The World of Async: Waiting Without Blocking

### The Analogy: The Coffee Shop ☕

To understand the difference between **Threads** and  **Async** , imagine a coffee shop.

1. **Synchronous (Blocking):** You order coffee. The cashier stands there, stares at the coffee machine, waits for it to brew, pours it, hands it to you, and *only then* takes the next customer's order.
   * *Problem:* Extremely slow.
2. **Threads (Parallelism):** You hire 10 cashiers. Now you can serve 10 people at once.
   * *Problem:* Expensive (wages/memory).
3. **Asynchronous (Concurrency):** One cashier takes your order, gives you a ticket, and immediately takes the next order. When the coffee is ready, they call your number.
   * *Benefit:* One person handles many requests efficiently.

---

## 1. Promises vs. Futures 🔮

In the programming world, when you ask for something that isn't ready yet (like downloading a file), you get a placeholder.

### JavaScript: The "Promise"

* **Concept:** A Promise is an object representing a value that may be available now, later, or never.
* **Behavior:** In JavaScript, Promises are  **"Eager"** . As soon as you create a Promise, the code starts running immediately in the background.

### Rust: The "Future"

* **Concept:** A `<span data-markdown-start-index="1560">Future</span>` is a trait (interface) for a value that will be ready later.
* **Behavior:** In Rust, Futures are  **"Lazy"** .
  * If you call an `<span data-markdown-start-index="1702">async</span>` function in Rust,  **nothing happens** .
  * The code inside does not run until you explicitly `<span data-markdown-start-index="1806">await</span>` it or give it to an "Executor" (Runtime).

> **Simple Rule:** A Rust `<span data-markdown-start-index="1882">Future</span>` is like a "To-Do List." Writing the list doesn't do the work. You have to actually start the list.

---

## 2. Coroutines & Async/Await ⏯️

How does the computer pause a function and come back later?

### Coroutines

A **Coroutine** is a function that can **suspend** (pause) its execution and **resume** later at the exact same spot, keeping its local variables intact.

* Normal Function: Starts -> Runs -> Returns.
* Coroutine: Starts -> Runs -> **Pauses (Yields)** -> Resumes -> Returns.

### Async/Await in Rust

Rust implements Asynchronous programming using a specific type of Coroutine.

* **`<span data-markdown-start-index="2485">async</span>` keyword:** Transforms a block of code into a state machine that implements the `<span data-markdown-start-index="2572">Future</span>` trait.
* **`<span data-markdown-start-index="2593">.await</span>` keyword:** This is the "Pause Button."
  * When code hits `<span data-markdown-start-index="2663">.await</span>`, it checks: "Is the data ready?"
  * **Yes:** Continue running.
  * **No:****Yield** control back to the system (Executor) so it can do other work while waiting.

```rust
// This function returns a Future. It doesn't run yet!
async fn get_data() -> String {
    "Data loaded".to_string()
}

async fn main_work() {
    println!("Start");
  
    // .await tells Rust: "Pause this function here until get_data is done.
    // Go do other work in the meantime."
    let data = get_data().await; 
  
    println!("Got: {}", data);
}
```

---

## 3. Goroutines vs. Tasks 🏃

How do we run these concurrent pieces of code?

### Go Language: "Goroutines"

Go uses "Green Threads."

* When you write `<span data-markdown-start-index="3353">go func()</span>`, Go spawns a lightweight thread.
* The **Go Runtime** (built into the language) automatically manages these. It effectively tricks the OS into thinking it's using few threads, while Go swaps thousands of Goroutines on and off them efficiently.

### Rust: "Tasks"

Rust **does not** have a built-in runtime in the standard library (to keep the language small). You must pull in a library (crate) like **Tokio** or  **Async-std** .

In Tokio, we have  **Tasks** :

* A **Task** is Rust's version of a Goroutine.
* It is a lightweight, non-blocking unit of execution.
* You create one using `<span data-markdown-start-index="3948">tokio::spawn</span>`.

**The Comparison:**

| Feature                 | Go (Goroutine)                                                 | Rust (Tokio Task)                                                              |
| ----------------------- | -------------------------------------------------------------- | ------------------------------------------------------------------------------ |
| **Creation**      | `<span data-markdown-start-index="4071">go my_func()</span>` | `<span data-markdown-start-index="4088">tokio::spawn(my_func())</span>`      |
| **Weight**        | Very lightweight                                               | Very lightweight                                                               |
| **Scheduling**    | Managed by Go Runtime                                          | Managed by Tokio Runtime                                                       |
| **Communication** | Channels                                                       | Channels (`<span data-markdown-start-index="4279">tokio::sync::mpsc</span>`) |

---

## 4. Putting it Together: The Mental Model

If you are writing a Rust program using the **Tokio** runtime, here is the hierarchy:

1. **The Runtime (Executor):** This is the engine (like the Coffee Shop Manager). It manages the threads.
2. **The Task:** This is a standalone job (like a Customer Order). You spawn it onto the Runtime.
3. **The Future:** This is the specific logic inside the task (The steps to make the coffee).
4. **Async/Await:** This is the syntax you use to define the Future.

### Example: A "Rust Goroutine" (Task)

```rust
// You need the 'tokio' crate in Cargo.toml
// [dependencies]
// tokio = { version = "1", features = ["full"] }

use tokio::time::{sleep, Duration};

#[tokio::main] // Starts the Tokio Runtime
async fn main() {
    // Spawn a lightweight task (Like a Goroutine)
    let task_handle = tokio::spawn(async {
        for i in1..4 {
            println!("Task says: {}", i);
            sleep(Duration::from_millis(50)).await; // Non-blocking sleep!
        }
    });

    println!("Main function is doing other work...");
  
    // Wait for the task to finish
    task_handle.await.unwrap();
}
```

## Summary Dictionary

| Term                | Simple Definition                                                                                              |
| ------------------- | -------------------------------------------------------------------------------------------------------------- |
| **Async**     | The concept of not waiting for tasks to finish before starting new ones.                                       |
| **Future**    | A data structure representing a value that will arrive later (Lazy).                                           |
| **Promise**   | (JavaScript term) Similar to Future, but starts executing immediately (Eager).                                 |
| **Coroutine** | A function that can pause and resume.`<span data-markdown-start-index="5819">async fn</span>`is a coroutine. |
| **Goroutine** | (Go term) A lightweight thread managed by Go.                                                                  |
| **Task**      | (Rust/Tokio term) A lightweight thread managed by a library like Tokio.                                        |


Here is a clear and comprehensive guide to  **Tokio** , the most popular runtime for writing asynchronous applications in Rust.

---

# 🦀 Tokio: The Engine of Async Rust

As we discussed, Rust's standard library provides the *syntax* for async programming (`<span data-markdown-start-index="254">async</span>`, `<span data-markdown-start-index="263">.await</span>`, `<span data-markdown-start-index="273">Future</span>`), but it does **not** provide the engine to run it.

**Tokio** is that engine. It is an event-driven, non-blocking I/O platform for writing asynchronous applications.

## 1. Why do we need Tokio?

In languages like Go or Node.js, the "runtime" (the background code that schedules tasks and handles network calls) is built into the language.

In Rust, the language is kept minimal.

* **Rust Standard Library:** Defines *what* a Future is.
* **Tokio:** Defines *how* to execute that Future.

Without Tokio (or a similar runtime like `<span data-markdown-start-index="810">async-std</span>`), calling an `<span data-markdown-start-index="835">async</span>` function does absolutely nothing.

---

## 2. Core Concepts of Tokio

### A. The Runtime (`<span data-markdown-start-index="930">#[tokio::main]</span>`)

You cannot use `<span data-markdown-start-index="962">.await</span>` inside a normal `<span data-markdown-start-index="987">fn main()</span>`. You need to start the Tokio engine first. The easiest way is using the macro.

```rust
#[tokio::main]
async fn main() {
    println!("Tokio is running!");
}
```

*Behind the scenes, this macro transforms your main function into a normal function that starts the Tokio runtime and blocks until your code finishes.*

### B. Tasks (`<span data-markdown-start-index="1320">tokio::spawn</span>`)

This is the heart of concurrency in Tokio.

* A **Task** is a "Green Thread."
* It is extremely lightweight. You can spawn **millions** of tasks on a single machine.
* The Tokio scheduler switches between tasks instantly whenever one of them hits an `<span data-markdown-start-index="1587">.await</span>` point (like waiting for a network request).

```rust
tokio::spawn(async {
    // This runs concurrently in the background
});
```

### C. Async I/O

Tokio provides asynchronous versions of standard I/O operations.

* **Instead of:**`<span data-markdown-start-index="1821">std::fs::File</span>`, `<span data-markdown-start-index="1838">std::net::TcpStream</span>`, `<span data-markdown-start-index="1861">std::thread::sleep</span>`
* **Use:**`<span data-markdown-start-index="1894">tokio::fs::File</span>`, `<span data-markdown-start-index="1913">tokio::net::TcpStream</span>`, `<span data-markdown-start-index="1938">tokio::time::sleep</span>`

**Critical Rule:** NEVER use blocking calls (like `<span data-markdown-start-index="2009">std::thread::sleep</span>`) inside a Tokio task. It will pause the entire thread, stopping all other tasks running on that thread. Always use `<span data-markdown-start-index="2145">tokio::time::sleep</span>`.

---

## 3. Practical Example: A Web Server Simulator

Let's simulate handling multiple requests at the same time without creating heavy OS threads.

```rust
use tokio::time::{sleep, Duration};

async fn handle_request(id: u32) {
    println!("Request {}: processing started...", id);
  
    // Simulate a slow database call (2 seconds)
    // We use tokio::sleep, NOT std::thread::sleep
    sleep(Duration::from_secs(2)).await; 
  
    println!("Request {}: processing DONE.", id);
}

#[tokio::main]
async fn main() {
    let mut handles = vec![];

    println!("Server starting...");

    for i in1..=5 {
        // Spawn a new task for each request.
        // These will all start virtually at the same time.
        let handle = tokio::spawn(handle_request(i));
        handles.push(handle);
    }

    // Wait for all tasks to complete
    for handle in handles {
        handle.await.unwrap();
    }
  
    println!("All requests finished!");
}
```

**Result:**
Even though each request takes 2 seconds, the whole program takes only **~2 seconds total** (not 10), because they wait in parallel.

---

## 4. Channels (`<span data-markdown-start-index="3262">tokio::sync::mpsc</span>`)

Just like standard threads, Tokio tasks communicate using channels.
Tokio's channels are `<span data-markdown-start-index="3370">async</span>`—sending or receiving can wait without blocking the thread.

* `<span data-markdown-start-index="3441">mpsc</span>`: Multi-Producer, Single-Consumer (Many senders, one receiver).
* `<span data-markdown-start-index="3514">oneshot</span>`: Sending a single value from one task to another.
* `<span data-markdown-start-index="3577">broadcast</span>`: sending values to *all* receivers.

```rust
use tokio::sync::mpsc;

#[tokio::main]
async fn main(){
    // Create a channel with a buffer of 32 messages
    let (tx, mut rx) = mpsc::channel(32);

    tokio::spawn(async move {
        tx.send("sending from a task").await.unwrap();
    });

    while let Some(message)= rx.recv().await {
        println!("GOT: {}", message);
    }
}
```

---

## 5. Shared State (`<span data-markdown-start-index="3989">tokio::sync::Mutex</span>`)

Tokio has its own Mutex: `<span data-markdown-start-index="4035">tokio::sync::Mutex</span>`.

* **Standard `<span data-markdown-start-index="4071">std::sync::Mutex</span>`:** Blocks the thread until the lock is ready. (Bad for async).
* **Tokio `<span data-markdown-start-index="4164">tokio::sync::Mutex</span>`:** The `<span data-markdown-start-index="4192">.lock()</span>` method is an `<span data-markdown-start-index="4215">async</span>` function. It waits *asynchronously* until the lock is ready, letting other tasks run in the meantime.

```rust
use tokio::sync::Mutex;
use std::sync::Arc;

let db = Arc::new(Mutex::new(vec![]));
let db_clone = db.clone();

tokio::spawn(async move {
    // We .await the lock!
    let mut data = db_clone.lock().await; 
    data.push("new entry");
});
```

---

## 6. Summary: When to use Tokio?

| Feature               | `<span data-markdown-start-index="4617">std</span>`(Standard Library)        | `<span data-markdown-start-index="4644">tokio</span>`                           |
| --------------------- | ------------------------------------------------------------------------------ | --------------------------------------------------------------------------------- |
| **Model**       | Synchronous (Blocking)                                                         | Asynchronous (Non-blocking)                                                       |
| **Concurrency** | OS Threads (`<span data-markdown-start-index="4775">1:1</span>`)             | Async Tasks (`<span data-markdown-start-index="4797">M:N</span>`)               |
| **Best For**    | CPU-heavy computations (Number crunching).                                     | I/O-heavy applications (Web servers, DBs, Networking).                            |
| **Sleeping**    | `<span data-markdown-start-index="4940">thread::sleep</span>`(Blocks thread) | `<span data-markdown-start-index="4974">tokio::time::sleep</span>`(Yields task) |

Tokio is the foundation for most of the Rust web ecosystem, including frameworks like  **Axum** ,  **Actix-web** , and  **Hyper** .

Does this explain why you need to add `<span data-markdown-start-index="5175">tokio</span>` to your `<span data-markdown-start-index="5191">Cargo.toml</span>` dependencies? 📦
