# 🦀 Rust Learning Journey

A comprehensive, hands-on learning repository covering Rust programming from fundamentals to advanced topics — including networking, async programming, macros, and system design.

---

## 📌 About This Project

This repository represents a structured learning path through the Rust programming language. Each folder contains working code, experiments, and notes built step-by-step — from basic syntax and ownership to TCP servers, async runtimes, distributed systems, and custom macros.

The goal is not just to learn Rust syntax, but to **deeply understand** systems programming, memory safety, concurrency, and how Rust's design philosophy solves real-world problems.

---

## 📁 Project Structure

```
.
├── 01_first/              # Rust basics — setup, syntax, types, variables
├── 02_second/             # Control flow, functions, expressions
├── 03_third/              # Ownership, borrowing, and the borrow checker
├── 04_four/               # Structs, enums, pattern matching
├── 05_five/               # Traits, generics, and polymorphism
├── 06_six/                # Error handling — Result, Option, ? operator
├── 07_seven/              # Collections — Vec, HashMap, iterators
├── 08_eight/              # Modules, crates, and project structure (mod, use, pub)
├── 09_nine/               # Closures, higher-order functions, lifetimes
├── 10_ten/                # Smart pointers — Box, Rc, Arc, RefCell
├── 11_eleven/             # Concurrency — threads, channels, shared state
├── Rust_VS_Python/        # Fibonacci calculator — Rust vs Python performance comparison
├── ASYNC_TOKIO/           # Async programming — futures, tokio, reqwest, clap CLI
├── MACRO_TEST/            # Macros — declarative (macro_rules!) and procedural
├── DHT/                   # Distributed Hash Table implementation
├── CS/                    # Computer Science concepts and algorithms
└── General_README/        # General notes and references
```

---

## 🗺️ Learning Path

### Phase 1 — Foundations (`01` → `07`)

These modules cover the core language features that every Rust programmer must understand.

| Folder      | Topic             | Key Concepts                                                 |
| ----------- | ----------------- | ------------------------------------------------------------ |
| `01_first`  | Basics            | Installation, cargo, variables, types,`fn main()`            |
| `02_second` | Control Flow      | `if/else`, loops,`match`, functions, expressions             |
| `03_third`  | Ownership         | Move semantics, borrowing,`&`and `&mut`, borrow checker      |
| `04_four`   | Data Structures   | Structs, enums,`Option`,`Some`,`None`, pattern matching      |
| `05_five`   | Traits & Generics | Trait definitions,`impl`, generics,`dyn Trait`               |
| `06_six`    | Error Handling    | `Result<T, E>`,`Option<T>`,`?`operator, custom errors        |
| `07_seven`  | Collections       | `Vec`,`HashMap`, iterators,`.map()`,`.filter()`,`.collect()` |

---

### Phase 2 — Intermediate (`08` → `11`)

Building on the foundations with more complex language features.

| Folder      | Topic                | Key Concepts                                                |
| ----------- | -------------------- | ----------------------------------------------------------- |
| `08_eight`  | Module System        | `mod`,`use`,`pub`,`crate`,`super`,`self`,`lib.rs`           |
| `09_nine`   | Closures & Lifetimes | `Fn`,`FnMut`,`FnOnce`, lifetime annotations `'a`            |
| `10_ten`    | Smart Pointers       | `Box<T>`,`Rc<T>`,`Arc<T>`,`RefCell<T>`, interior mutability |
| `11_eleven` | Concurrency          | `std::thread`,`JoinHandle`,`Mutex`,`Arc`, channels          |

---

### Phase 3 — Projects & Advanced Topics

Real-world projects combining everything learned in Phases 1 and 2.

#### 🐍 `Rust_VS_Python` — Performance Comparison

A Fibonacci calculator implemented in both Rust and Python, designed to compare performance and showcase language differences.

- **Iterative approach** for efficiency
- Handles **arbitrarily large numbers** using `num-bigint` crate (`BigUint`)
- Solves **integer overflow** issues that occur with `u128` for large inputs (e.g., `fib(500)`)
- **Execution time measurement** using `Instant` (Rust) and `perf_counter` (Python)
- Demonstrates Rust's **5–10x speed advantage** over Python for computation

  **Key Rust Concepts** :

- `BigUint` for arbitrary precision integers
- `Instant` and `Duration` for high-precision timing
- Iterative computation with mutable variables
- Cargo dependency management (`num-bigint`)

---

#### 🌐 `ASYNC_TOKIO` — Async TCP Server & CLI Downloader

An async networking project combining multiple advanced topics.

**TCP Server (`test-tcp-server`)** :

- Multi-client async TCP server using `async-std`
- Spawns a **task per connection** (not a thread — lightweight and scalable)
- Uses `StreamExt` for async iteration over incoming connections
- Demonstrates `TcpListener`, `TcpStream`, `write_all`, `peer_addr`
- Non-blocking I/O — thousands of connections on a single thread

  **Download Manager (`test-tokio`)** :

- CLI argument parsing with **clap** (`#[derive(Parser)]`)
- HTTP HEAD request with **reqwest** to check server capabilities
- Checks `Accept-Ranges: bytes` header for **resumable download** support
- Custom error types with `enum MyError`
- Proper error propagation using `?` operator and `map_err`
- Supports `--url`, `--connections`, and `--file` arguments

  **Key Rust Concepts** :

- `async/await` syntax
- `futures::join!` for concurrent execution
- `block_on` vs `#[tokio::main]` for running async code
- Futures, Tasks, Executors, and Reactors
- `Result<T, E>` error handling in async context
- `async-std` vs `tokio` runtimes

---

#### 🔧 `MACRO_TEST` — Rust Macros

A hands-on exploration of Rust's macro system.

**Custom Macro: `printlines!`** :

- Accepts **any number of arguments** of **any type**
- Prints each value with a label using repetition patterns
- Defined in `lib.rs`, exported with `#[macro_export]`, imported in `main.rs`

  **Concepts Covered** :

- `macro_rules!` syntax and structure
- **Fragment specifiers** : `expr`, `ident`, `ty`, `stmt`, `item`, `pat`, `tt`, and more
- **Repetition patterns** : `$(...)*` (zero or more), `$(...)+` (one or more), `$(...)?` (optional)
- Separators in repetition (commas, semicolons)
- Macro hygiene and scoping
- `#[macro_export]` for cross-module macro visibility
- **Declarative vs Procedural** macros comparison
- Real-world procedural macro examples: `clap` (`#[derive(Parser)]`), `tokio` (`#[tokio::main]`)

---

#### 💻 `CS` — Computer Science Concepts

Fundamental algorithms and data structures implemented in Rust.

---

#### 🌐 `DHT` — Distributed Hash Table

Implementation and exploration of Distributed Hash Table concepts — a core building block for peer-to-peer networks, decentralized storage, and distributed systems.

---

## 🛠️ Tech Stack

| Category        | Tools / Crates           |
| --------------- | ------------------------ |
| Language        | Rust (Edition 2021)      |
| Build System    | Cargo                    |
| Async Runtime   | Tokio, async-std         |
| HTTP Client     | reqwest                  |
| CLI Framework   | clap                     |
| Big Integers    | num-bigint               |
| Async Utilities | futures                  |
| Networking      | std::net, async-std::net |
| Version Control | Git                      |

---

## ⚙️ Setup & Requirements

### Prerequisites

- **Rust** — Install via [rustup](https://rustup.rs/)
- **Cargo** — Comes with rustup
- **Git** — For cloning the repo

### Installation

```bash
# Clone the repository
git clone <your-github-repo-url>.git
cd rust-learning-journey

# Navigate to any project folder
cd ASYNC_TOKIO

# Build
cargo build

# Run
cargo run

# Run specific binary (if multiple exist)
cargo run --bin test-tokio -- --url "https://example.com/file"
```

### Verify Rust Installation

```bash
rustc --version
cargo --version
```

---

## 🏗️ How to Navigate This Repo

1. **Start with `01_first`** — Set up your environment and write your first Rust program
2. **Follow the numbered folders in order** — Each builds on the previous
3. **After `11_eleven`** , move to the project folders (`Rust_VS_Python`, `ASYNC_TOKIO`, `MACRO_TEST`)
4. **Read comments in the code** — They explain concepts and reasoning
5. **Experiment!** — Modify the code, break things, and learn from the compiler errors

---

## 📊 Progress Tracker

| #   | Topic                           | Status         |
| --- | ------------------------------- | -------------- |
| 01  | Basics & Setup                  | ✅ Done        |
| 02  | Control Flow & Functions        | ✅ Done        |
| 03  | Ownership & Borrowing           | ✅ Done        |
| 04  | Structs, Enums & Matching       | ✅ Done        |
| 05  | Traits & Generics               | ✅ Done        |
| 06  | Error Handling                  | ✅ Done        |
| 07  | Collections & Iterators         | ✅ Done        |
| 08  | Module System                   | ✅ Done        |
| 09  | Closures & Lifetimes            | ✅ Done        |
| 10  | Smart Pointers                  | ✅ Done        |
| 11  | Concurrency & Threads           | ✅ Done        |
| —   | Rust vs Python (Fibonacci)      | ✅ Done        |
| —   | Async TCP Server                | ✅ Done        |
| —   | CLI Downloader (clap + reqwest) | ✅ Done        |
| —   | Macros (Declarative)            | ✅ Done        |
| —   | DHT                             | 🔄 In Progress |
| —   | Computer Science                | 🔄 In Progress |

---

## 🔑 Key Rust Concepts Covered

### Ownership & Memory Safety

- Move semantics and the ownership model
- Borrowing with `&` and `&mut`
- The borrow checker — Rust's compile-time memory safety guarantee
- No garbage collector, no dangling pointers, no data races

### Concurrency

- OS threads with `std::thread`
- Shared state with `Arc<Mutex<T>>`
- Async concurrency with `async/await`
- Tasks vs Threads — lightweight vs heavyweight
- Executors and Reactors

### Networking

- TCP server/client with `std::net` (synchronous)
- Async TCP with `async-std::net`
- HTTP requests with `reqwest`
- Socket programming concepts: bind, listen, accept
- IP addressing: `127.0.0.1`, `0.0.0.0`, ports

### Macros

- Declarative macros with `macro_rules!`
- Procedural macros (derive, attribute, function-like)
- Real-world macro usage: `println!`, `vec!`, `#[derive(...)]`, `#[tokio::main]`

### Error Handling

- `Result<T, E>` and `Option<T>`
- The `?` operator for error propagation
- Custom error types with enums
- `map_err`, `unwrap_or`, `ok_or`

---

## 📚 Useful Resources

| Resource              | Link                                                                            |
| --------------------- | ------------------------------------------------------------------------------- |
| The Rust Book         | [doc.rust-lang.org/book](https://doc.rust-lang.org/book/)                       |
| Rust by Example       | [doc.rust-lang.org/rust-by-example](https://doc.rust-lang.org/rust-by-example/) |
| Rustlings (Exercises) | [github.com/rust-lang/rustlings](https://github.com/rust-lang/rustlings)        |
| Rust Playground       | [play.rust-lang.org](https://play.rust-lang.org/)                               |
| Async Book            | [rust-lang.github.io/async-book](https://rust-lang.github.io/async-book/)       |
| Crates.io             | [crates.io](https://crates.io/)                                                 |

---

## 💡 Why Rust?

Rust was chosen for this learning journey because it fundamentally changes how you think about programming:

- **Memory Safety without GC** — You understand _exactly_ when memory is allocated and freed
- **Zero-Cost Abstractions** — High-level code compiles to highly optimized machine code
- **Fearless Concurrency** — The compiler prevents data races at compile time
- **Great Tooling** — `cargo`, `rustfmt`, `clippy`, and incredible error messages
- **Transferable Knowledge** — Understanding Rust makes you a better programmer in _any_ language

---

## 🏷️ License

This project is for **educational purposes** . Feel free to use, modify, and learn from any of the code here.

---

_Built with dedication and a lot of compiler error messages 🦀_
