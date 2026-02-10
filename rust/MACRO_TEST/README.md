# Rust Macros - Complete Comprehensive Guide

## Table of Contents

1. [What is a Macro?](https://claude.ai/chat/d8a349a5-1109-437b-b4a9-588498bf20cb#what-is-a-macro)
2. [Types of Macros](https://claude.ai/chat/d8a349a5-1109-437b-b4a9-588498bf20cb#types-of-macros)
3. [Code Overview](https://claude.ai/chat/d8a349a5-1109-437b-b4a9-588498bf20cb#code-overview)
4. [main.rs Walkthrough](https://claude.ai/chat/d8a349a5-1109-437b-b4a9-588498bf20cb#mainrs-walkthrough)
5. [lib.rs Walkthrough](https://claude.ai/chat/d8a349a5-1109-437b-b4a9-588498bf20cb#librs-walkthrough)
6. [macro_rules! Syntax Deep Dive](https://claude.ai/chat/d8a349a5-1109-437b-b4a9-588498bf20cb#macro_rules-syntax-deep-dive)
7. [Pattern Matching in Macros](https://claude.ai/chat/d8a349a5-1109-437b-b4a9-588498bf20cb#pattern-matching-in-macros)
8. [Macro Expansion](https://claude.ai/chat/d8a349a5-1109-437b-b4a9-588498bf20cb#macro-expansion)
9. [#[macro_export] Explained](https://claude.ai/chat/d8a349a5-1109-437b-b4a9-588498bf20cb#macro_export-explained)
10. [Common Built-in Macros](https://claude.ai/chat/d8a349a5-1109-437b-b4a9-588498bf20cb#common-built-in-macros)
11. [Procedural Macros](https://claude.ai/chat/d8a349a5-1109-437b-b4a9-588498bf20cb#procedural-macros)
12. [Complete Examples](https://claude.ai/chat/d8a349a5-1109-437b-b4a9-588498bf20cb#complete-examples)
13. [Macros vs Functions](https://claude.ai/chat/d8a349a5-1109-437b-b4a9-588498bf20cb#macros-vs-functions)
14. [Best Practices](https://claude.ai/chat/d8a349a5-1109-437b-b4a9-588498bf20cb#best-practices)
15. [Troubleshooting](https://claude.ai/chat/d8a349a5-1109-437b-b4a9-588498bf20cb#troubleshooting)

---

## What is a Macro?

A macro is **code that generates code** at compile time.

**Simple analogy** :

- A **function** runs code at runtime
- A **macro** writes code at compile time, then that code runs

```rust
// Function: runs at runtime
fn add(a: i32, b: i32) -> i32 {
    a + b
}

// Macro: generates code at compile time
macro_rules! add {
    ($a:expr, $b:expr) => {
        $a + $b
    };
}

// Usage
let x = add(1, 2);   // Function call
let y = add!(1, 2);  // Macro call (notice the !)
```

**Why macros exist?**

- Functions can't change how code is structured
- Macros can generate completely new code
- Macros can accept different types without generics
- Some things simply can't be done with functions

**How to identify a macro?**

- Always ends with `!` (exclamation mark)
- `println!`, `vec!`, `format!`, `macro_rules!`

---

## Types of Macros

### Comment: `//MACRO`

### Comment: `//Declarative -> println!,vec,format, ...`

### Comment: `//Procedural -> function-like, Derive, Attribute,clap,tokio`

Rust has **two main types** of macros:

---

### 1. Declarative Macros (`macro_rules!`)

**Also called** : "macro by example", "MBE"

**What they are** : You define patterns and what code to generate for each pattern

**Syntax** : `macro_rules! name { ... }`

**Examples** : `println!`, `vec!`, `format!`, `panic!`, `assert!`

**Your custom macro `printlines!` is a declarative macro!**

```rust
// Declarative macro definition
macro_rules! say_hello {
    () => {
        println!("Hello!");
    };
}

// Usage
say_hello!();  // Prints: Hello!
```

**How it works** :

1. You write patterns (like templates)
2. When you call the macro, Rust matches your input to a pattern
3. Rust generates code based on that pattern
4. The generated code is compiled

---

### 2. Procedural Macros (`proc-macro`)

**Also called** : "compiler plugins"

**What they are** : Written as Rust functions that manipulate code tokens

**Three subtypes** :

#### A. Function-like Macros

```rust
// Looks like declarative macros but written as functions
#[proc_macro]
pub fn my_macro(input: TokenStream) -> TokenStream {
    // Process input tokens
    // Return new tokens
}

// Usage
my_macro!(some code here);
```

#### B. Derive Macros

```rust
// Automatically implement traits
#[proc_macro_derive(MyTrait)]
pub fn my_derive(input: TokenStream) -> TokenStream {
    // Generate trait implementation
}

// Usage
#[derive(MyTrait)]  // Automatically implements MyTrait
struct MyStruct {}
```

**Common derive macros** : `Debug`, `Clone`, `PartialEq`, `Serialize`, `Deserialize`

#### C. Attribute Macros

```rust
// Custom attributes that transform code
#[proc_macro_attribute]
pub fn my_attribute(attr: TokenStream, item: TokenStream) -> TokenStream {
    // Transform the item
}

// Usage
#[my_attribute]
fn some_function() {}
```

**Examples from your comments** :

- `clap` → Uses derive + attribute macros (`#[derive(Parser)]`, `#[clap(...)]`)
- `tokio` → Uses attribute macro (`#[tokio::main]`)

---

### Comparison Table

| Feature      | Declarative       | Procedural                          |
| ------------ | ----------------- | ----------------------------------- |
| Keyword      | `macro_rules!`    | `#[proc_macro]`                     |
| Written in   | Pattern matching  | Rust functions                      |
| Complexity   | Medium            | High                                |
| Performance  | Fast              | Slower compile                      |
| Flexibility  | Good              | Maximum                             |
| Examples     | `println!`,`vec!` | `#[derive(Debug)]`,`#[tokio::main]` |
| Your example | `printlines!`     | `clap`,`tokio`                      |

---

## Code Overview

### Project Structure

```
MACRO_TEST/
├── Cargo.toml
└── src/
    ├── main.rs      # Uses the macro
    └── lib.rs       # Defines the macro
```

### What the Code Does

1. **lib.rs** defines a custom macro called `printlines!`
2. **main.rs** imports and uses `printlines!`
3. The macro takes multiple values and prints each one
4. It works with **any type** that implements `Display`

---

## main.rs Walkthrough

```rust
//MACRO
//Declarative -> println!,vec,format, ...
//Procedural -> function-like, Derive, Attribute,clap,tokio

use MACRO_TEST::printlines;
fn main() {
    printlines!("Hi", 10);
}
```

### Line-by-Line:

---

#### Comments (Lines 1-3)

```rust
//MACRO
//Declarative -> println!,vec,format, ...
//Procedural -> function-like, Derive, Attribute,clap,tokio
```

These are notes about the **two types of macros** in Rust:

- **Declarative** : Pattern-based macros like `println!`, `vec!`, `format!`
- **Procedural** : Code-generating macros like `clap` (Parser derive), `tokio` (main attribute)

---

#### Import Statement

```rust
use MACRO_TEST::printlines;
```

**Breakdown** :

- `MACRO_TEST` = Your crate name (defined in `Cargo.toml`)
- `printlines` = The macro you're importing
- `use` = Brings the macro into scope

**Why import a macro?**

- Macros defined in `lib.rs` with `#[macro_export]` are exported at the **crate root**
- You need to `use` them just like any other item
- Without this line, `printlines!` would not be available in `main.rs`

**What is `MACRO_TEST`?**
Your `Cargo.toml` probably looks like:

```toml
[package]
name = "MACRO_TEST"   # or "macro_test" or "macro-test"
version = "0.1.0"
edition = "2021"
```

**Note** : Rust converts crate names:

- `macro-test` → `macro_test` (hyphens become underscores)
- `MACRO_TEST` stays as `MACRO_TEST` if that's the name

---

#### Main Function

```rust
fn main() {
    printlines!("Hi", 10);
}
```

**Breakdown** :

- `printlines!` = Calling the macro (note the `!`)
- `"Hi"` = First argument (type: `&str`)
- `10` = Second argument (type: `i32`)
- Both are **expressions** (`expr` in macro terms)

  **What happens at compile time** :

```rust
// What you wrote:
printlines!("Hi", 10);

// What Rust generates (expands to):
{
    println!("Value is:");
    println!("{}", "Hi");
    println!();

    println!("Value is:");
    println!("{}", 10);
    println!();
}
```

**Output** :

```
Value is:
Hi

Value is:
10

```

---

## lib.rs Walkthrough

```rust
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
```

### Line-by-Line:

---

#### Comments (Lines 1-5)

```rust
//printlines!("Hi" , 10 , false);
```

This shows an **example usage** of the macro with 3 arguments.

```rust
//println!("{}", "HI");
//println!("{}", 10);
//println!("{}", false);
```

This shows what the macro **expands to** for those 3 arguments. Each argument gets its own `println!` call. This is the **manual version** of what the macro does automatically!

**This is the key insight** : The macro replaces ONE line with MANY lines of code.

---

#### `#[macro_export]`

```rust
#[macro_export]
```

**What it does** :

- Makes the macro **public** (accessible outside this crate)
- Exports it at the **crate root** (top level)
- Required if you want to use the macro from `main.rs` or other crates

  **Without `#[macro_export]`** :

```rust
// lib.rs
macro_rules! printlines { ... }  // Private! Only usable in lib.rs
```

**With `#[macro_export]`** :

```rust
// lib.rs
#[macro_export]
macro_rules! printlines { ... }  // Public! Usable anywhere

// main.rs
use MACRO_TEST::printlines;  // Can import it!
```

**Note** : `#[macro_export]` always puts the macro at the **crate root** , regardless of where you define it. Even if you define it inside a module, it's exported at the top level.

---

#### `macro_rules!`

```rust
macro_rules! printlines {
```

**Breakdown** :

- `macro_rules!` = The keyword to define a declarative macro
- `printlines` = The name of your macro
- Users call it as `printlines!(...)`

**Why is `macro_rules` itself a macro?**

- Notice `macro_rules!` ends with `!`
- It's a **built-in compiler macro** used to define other macros
- It's special - handled directly by the Rust compiler

---

#### The Pattern and Body

```rust
    ($($line:expr),*) => {
        { $(
            println!("Value is:");
            println!("{}", $line);
            println!();
        )*}
    };
```

This is the **heart** of the macro. Let's break it down completely:

---

## macro_rules! Syntax Deep Dive

### General Structure

```rust
macro_rules! macro_name {
    (PATTERN) => { EXPANSION };
    (PATTERN) => { EXPANSION };
    // Can have multiple patterns!
}
```

- **PATTERN** : What the macro matches (input)
- **EXPANSION** : What code to generate (output)
- `=>` means "expand to"
- `;` separates each rule

---

### Your Macro Pattern Explained

```rust
($($line:expr),*)
```

Let's break this down **character by character** :

#### `$(...)*` - Repetition

```
$( ... )*
^  ^^^  ^
|  |||  |
|  |||  └── Repeat zero or more times
|  ||└──── Content to repeat
|  |└───── Opening of repetition group
|  └────── Dollar sign = start of repetition
└───────── This whole thing means "match this pattern multiple times"
```

**Repetition markers** :

- `*` = zero or more times
- `+` = one or more times
- `?` = zero or one time

#### `$line:expr` - Capture Variable

```
$line:expr
^^^^^:^^^^
|||||:||||
|||||:└───── Fragment type (what kind of thing to match)
|||||:
└────:────── Variable name (what to call the captured value)
```

**`$line`** :

- `$` = prefix for macro variables
- `line` = the name you chose for this captured value
- You use `$line` later in the expansion to refer to it

**`:expr`** = Fragment specifier. Tells Rust what kind of code to match.

#### `,` - Separator

The comma between repetitions:

```rust
($($line:expr),*)
//            ^
//            separator between repeated items
```

This means: "match expressions separated by commas"

---

### Fragment Specifiers (`:expr` and others)

| Specifier  | Matches        | Example                                     |
| ---------- | -------------- | ------------------------------------------- |
| `expr`     | Any expression | `1 + 2`,`"hello"`,`true`                    |
| `ident`    | Identifier     | `my_var`,`foo`                              |
| `ty`       | Type           | `i32`,`String`,`Vec<u8>`                    |
| `stmt`     | Statement      | `let x = 5;`                                |
| `item`     | Item           | `fn foo() {}`,`struct Bar`                  |
| `pat`      | Pattern        | `Some(x)`,`1..=5`                           |
| `path`     | Path           | `std::io::Result`                           |
| `tt`       | Token tree     | Any single token or `(...)`,`[...]`,`{...}` |
| `block`    | Block          | `{ ... }`                                   |
| `literal`  | Literal        | `42`,`"hello"`,`true`                       |
| `lifetime` | Lifetime       | `'a`,`'static`                              |
| `vis`      | Visibility     | `pub`,`pub(crate)`                          |
| `meta`     | Meta item      | Content of `#[...]`attributes               |

**In your macro** : `expr` matches any expression - that's why it works with `"Hi"`, `10`, `false`, or anything else!

---

### The Expansion Part Explained

```rust
{ $(
    println!("Value is:");
    println!("{}", $line);
    println!();
)*}
```

**Breakdown** :

#### Outer `{ ... }`

```rust
{ ... }
```

- Wraps everything in a **block**
- Makes the expansion a single expression
- Prevents variable name conflicts

#### `$( ... )*` - Expansion Repetition

```rust
$( ... )*
```

- Repeats the code inside for **each matched item**
- Must match the same repetition pattern from the input

#### `$line` - Using the Captured Value

```rust
println!("{}", $line);
//             ^^^^^
//             Insert the captured expression here
```

- Each time the repetition runs, `$line` is replaced with the next matched expression

---

## Pattern Matching in Macros

### How Rust Matches Your Input

When you call:

```rust
printlines!("Hi", 10);
```

Rust matches against the pattern `($($line:expr),*)`:

```
Input:   "Hi"  ,  10
Pattern: expr  ,  expr
         ^^^^     ^^^^
         $line    $line
         (1st)    (2nd)
```

**Step by step** :

1. `"Hi"` matches `expr` → stored as first `$line`
2. `,` matches the separator `,`
3. `10` matches `expr` → stored as second `$line`
4. No more input → repetition stops (`*` = zero or more)

---

### Multiple Pattern Rules

Macros can have **multiple patterns** :

```rust
macro_rules! my_macro {
    // Pattern 1: no arguments
    () => {
        println!("No arguments!");
    };

    // Pattern 2: one argument
    ($val:expr) => {
        println!("One: {}", $val);
    };

    // Pattern 3: two arguments
    ($a:expr, $b:expr) => {
        println!("Two: {} and {}", $a, $b);
    };

    // Pattern 4: variable arguments (like yours)
    ($($val:expr),*) => {
        $(println!("{}", $val);)*
    };
}

// Usage:
my_macro!();                // Matches Pattern 1
my_macro!(42);              // Matches Pattern 2
my_macro!(1, 2);            // Matches Pattern 3
my_macro!(1, 2, 3, 4, 5);  // Matches Pattern 4
```

**Rust tries patterns in order** - first match wins!

---

## Macro Expansion

### What Happens at Compile Time

Your code goes through these stages:

```
1. Source Code (what you write)
        ↓
2. Macro Expansion (macros replaced with generated code)
        ↓
3. Compilation (compiler compiles the expanded code)
        ↓
4. Binary (executable)
```

### Your Macro Expansion Step by Step

**You write** :

```rust
printlines!("Hi", 10);
```

**Rust expands to** :

```rust
{
    // First iteration ($line = "Hi")
    println!("Value is:");
    println!("{}", "Hi");
    println!();

    // Second iteration ($line = 10)
    println!("Value is:");
    println!("{}", 10);
    println!();
}
```

**With 3 arguments** (`printlines!("Hi", 10, false)`):

```rust
{
    // First iteration ($line = "Hi")
    println!("Value is:");
    println!("{}", "Hi");
    println!();

    // Second iteration ($line = 10)
    println!("Value is:");
    println!("{}", 10);
    println!();

    // Third iteration ($line = false)
    println!("Value is:");
    println!("{}", false);
    println!();
}
```

### Visualize Expansion in VS Code

You can see the expanded code:

- Press `Ctrl+Shift+P`
- Type: "rust-analyzer: Expand Macro"
- Place cursor on `printlines!`
- See the generated code!

---

## #[macro_export] Explained

### Why It's Needed

```
lib.rs defines macro
        ↓
#[macro_export] puts it at crate root
        ↓
main.rs imports with `use CRATE_NAME::macro_name`
        ↓
main.rs can use the macro
```

### Without `#[macro_export]`

```rust
// lib.rs
mod utils {
    macro_rules! my_macro { ... }  // Private to this module
}

// main.rs
// my_macro!();  // ERROR: can't find macro
```

### With `#[macro_export]`

```rust
// lib.rs
mod utils {
    #[macro_export]
    macro_rules! my_macro { ... }  // Exported to crate root!
}

// main.rs
use MY_CRATE::my_macro;  // Works! (imported from crate root)
my_macro!();              // Works!
```

**Important** : `#[macro_export]` always exports to crate root, even inside nested modules!

---

## Common Built-in Macros

### `println!`

```rust
println!("Hello");              // No arguments
println!("{}", 42);             // One argument
println!("{} {}", "Hi", 42);   // Multiple arguments
println!("{:?}", vec![1,2,3]); // Debug format
```

**Similar to your `printlines!` but** :

- Built into Rust
- Handles format strings
- Prints only one line per call

---

### `vec!`

```rust
let v = vec![1, 2, 3, 4, 5];

// Expands to something like:
let v = {
    let mut temp = Vec::new();
    temp.push(1);
    temp.push(2);
    temp.push(3);
    temp.push(4);
    temp.push(5);
    temp
};
```

**Uses repetition** just like your macro!

---

### `format!`

```rust
let s = format!("Hello, {}!", "world");
// s = "Hello, world!"
```

---

### `assert!` and `assert_eq!`

```rust
assert!(1 + 1 == 2);           // Panics if false
assert_eq!(1 + 1, 2);          // Panics if not equal
assert_ne!(1, 2);              // Panics if equal
```

---

### `panic!`

```rust
panic!("Something went wrong!");
// Crashes the program with a message
```

---

### `todo!` and `unimplemented!`

```rust
fn not_yet() {
    todo!();           // Placeholder - panics if called
}

fn also_not_yet() {
    unimplemented!();  // Same but different message
}
```

---

## Procedural Macros

### From Your Comments: `clap` and `tokio`

#### `clap` - Uses Derive + Attribute Macros

```rust
use clap::Parser;

#[derive(Parser)]           // ← Derive macro (generates argument parsing code)
struct Args {
    #[clap(short = 'u')]    // ← Attribute macro (configures the argument)
    url: String,
}

fn main() {
    let args = Args::parse();  // Generated by the derive macro!
}
```

**What happens** :

1. `#[derive(Parser)]` generates `parse()` method for `Args`
2. `#[clap(...)]` configures how each field is parsed
3. All this code is generated at **compile time**

---

#### `tokio` - Uses Attribute Macro

```rust
#[tokio::main]              // ← Attribute macro
async fn main() {
    println!("Async main!");
}

// Expands to something like:
fn main() {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(async {
        println!("Async main!");
    });
}
```

**What happens** :

1. `#[tokio::main]` transforms your `async fn main()`
2. Generates a regular `fn main()` with runtime setup
3. Wraps your async code inside it

---

### Writing a Procedural Macro (Advanced)

Procedural macros must be in a **separate crate** with `proc-macro` type:

```toml
# Cargo.toml
[lib]
proc-macro = true

[dependencies]
syn = "2"          # Parse Rust code
quote = "1"        # Generate Rust code
proc-macro2 = "1"  # Token manipulation
```

```rust
// lib.rs
use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(MyPrint)]
pub fn my_print_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    let name = &ast.ident;  // Get struct name

    let gen = quote! {
        impl MyPrint for #name {
            fn my_print(&self) {
                println!("I am {}", stringify!(#name));
            }
        }
    };

    gen.into()
}
```

```rust
// Usage in another crate
#[derive(MyPrint)]
struct Dog;

fn main() {
    let d = Dog;
    d.my_print();  // Prints: "I am Dog"
}
```

---

## Complete Examples

### Example 1: Your Macro with More Types

```rust
// lib.rs
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

// main.rs
use MACRO_TEST::printlines;

fn main() {
    printlines!("Hello", 42, 3.14, true, 'A');
}
```

**Output** :

```
Value is:
Hello

Value is:
42

Value is:
3.14

Value is:
true

Value is:
A

```

---

### Example 2: Macro with Multiple Patterns

```rust
#[macro_export]
macro_rules! smart_print {
    // Pattern 1: No arguments
    () => {
        println!("(empty)");
    };

    // Pattern 2: Single value with label
    ($label:expr => $val:expr) => {
        println!("{}: {}", $label, $val);
    };

    // Pattern 3: Multiple values
    ($($val:expr),+) => {  // Note: + means one or more
        $(
            println!("{}", $val);
        )+
    };
}

fn main() {
    smart_print!();                          // Pattern 1
    smart_print!("Name" => "Ali");           // Pattern 2
    smart_print!(1, 2, 3);                   // Pattern 3
}
```

**Output** :

```
(empty)
Name: Ali
1
2
3
```

---

### Example 3: Creating a HashMap Macro (like vec! but for maps)

```rust
use std::collections::HashMap;

#[macro_export]
macro_rules! hashmap {
    ($($key:expr => $value:expr),* $(,)?) => {{
        let mut map = std::collections::HashMap::new();
        $(
            map.insert($key, $value);
        )*
        map
    }};
}

fn main() {
    let scores = hashmap! {
        "Alice" => 100,
        "Bob"   => 95,
        "Ali"   => 88,
    };

    println!("{:?}", scores);
}
```

**Note** : `$(,)?` at the end allows an optional trailing comma!

---

### Example 4: Repeat with Index

```rust
#[macro_export]
macro_rules! print_indexed {
    ($($val:expr),*) => {
        {
            let mut index = 0;
            $(
                println!("[{}] {}", index, $val);
                index += 1;
            )*
        }
    };
}

fn main() {
    print_indexed!("first", "second", "third");
}
```

**Output** :

```
[0] first
[1] second
[2] third
```

---

### Example 5: Macro that Creates Functions

```rust
macro_rules! create_function {
    ($name:ident, $value:expr) => {
        fn $name() -> i32 {
            $value
        }
    };
}

// Creates functions at compile time!
create_function!(get_ten, 10);
create_function!(get_twenty, 20);

fn main() {
    println!("{}", get_ten());    // 10
    println!("{}", get_twenty()); // 20
}
```

**This is powerful!** Macros can create entire functions, structs, and more!

---

## Macros vs Functions

### Why Can't We Just Use a Function?

```rust
// Attempt with function (DOESN'T WORK for variable args with mixed types)
fn printlines(values: ???) {  // What type? Can't mix types!
    // ...
}

// With macro (WORKS!)
macro_rules! printlines {
    ($($line:expr),*) => { ... };  // Accepts ANY type!
}
```

### Comparison

| Feature          | Function              | Macro                    |
| ---------------- | --------------------- | ------------------------ |
| When runs        | Runtime               | Compile time             |
| Type checking    | Strict                | Flexible                 |
| Variable args    | Only with slices/Vec  | Native support           |
| Mixed types      | No (without generics) | Yes!                     |
| Can create items | No                    | Yes (functions, structs) |
| Hygiene          | Yes                   | Partial                  |
| Debugging        | Easy                  | Harder                   |
| Performance      | Same                  | Same (after expansion)   |
| Syntax           | `foo()`               | `foo!()`                 |

### When to Use Which

**Use functions when** :

- Simple logic
- Single type
- Easy to debug
- Don't need code generation

  **Use macros when** :

- Need variable number of arguments
- Need to accept mixed types
- Need to generate code/items
- Need special syntax (like `vec![]`)
- Performance-critical repeated patterns

---

## Best Practices

### 1. Document Your Macros

````rust
/// Prints each value on its own line with a label.
///
/// # Examples
/// ```
/// printlines!("hello", 42, true);
/// ```
#[macro_export]
macro_rules! printlines {
    ($($line:expr),*) => { ... };
}
````

---

### 2. Use Blocks in Expansion

```rust
// Bad - can leak variables
macro_rules! bad {
    ($val:expr) => {
        let x = $val;
        println!("{}", x);
    };
}

// Good - block prevents leaking
macro_rules! good {
    ($val:expr) => {{
        let x = $val;
        println!("{}", x);
    }};
}
```

---

### 3. Handle Trailing Commas

```rust
macro_rules! my_vec {
    ($($val:expr),* $(,)?) => {  // $(,)? = optional trailing comma
        {
            let mut v = Vec::new();
            $(v.push($val);)*
            v
        }
    };
}

// Both work!
let v1 = my_vec![1, 2, 3];
let v2 = my_vec![1, 2, 3,];  // trailing comma OK!
```

---

### 4. Use Descriptive Names

```rust
// Bad
macro_rules! p { ... }

// Good
macro_rules! print_all { ... }
```

---

### 5. Prefer Functions When Possible

```rust
// If you can do it with a function, do it!
fn print_value(val: &dyn std::fmt::Display) {
    println!("{}", val);
}

// Only use macros when functions can't do the job
macro_rules! print_all {
    ($($val:expr),*) => { ... };
}
```

---

## Troubleshooting

### Error: "macro not found"

```
error[E0463]: can't find crate for `MACRO_TEST`
```

**Cause** : Crate name mismatch or macro not exported.

**Solution 1** : Check `Cargo.toml` name matches your `use` statement:

```toml
[package]
name = "MACRO_TEST"  # Must match what you use in main.rs
```

**Solution 2** : Make sure `#[macro_export]` is on the macro:

```rust
#[macro_export]  // Without this, macro is private!
macro_rules! printlines { ... }
```

**Solution 3** : Check your import:

```rust
use MACRO_TEST::printlines;  // Crate name must match exactly
```

---

### Error: "unexpected token"

```
error: unexpected token in macro invocation
```

**Cause** : Wrong separator or missing comma.

**Bad** :

```rust
printlines!("Hi" 10);  // Missing comma!
```

**Good** :

```rust
printlines!("Hi", 10);  // Comma separator
```

---

### Error: "doesn't implement Display"

```
error[E0277]: `MyStruct` doesn't implement `std::fmt::Display`
```

**Cause** : Passing a type that can't be printed with `{}`.

**Solution 1** : Use `{:?}` (Debug) instead of `{}` (Display):

```rust
#[macro_export]
macro_rules! printlines {
    ($($line:expr),*) => {
        { $(
            println!("Value is:");
            println!("{:?}", $line);  // Debug format works with more types
            println!();
        )*}
    };
}
```

**Solution 2** : Derive Debug on your struct:

```rust
#[derive(Debug)]
struct MyStruct {
    name: String,
}

printlines!(MyStruct { name: String::from("test") });
```

---

### Error: "expected expression"

```
error: expected expression
```

**Cause** : Usually a syntax error in macro definition.

**Common mistakes** :

```rust
// Bad: missing block braces in expansion
macro_rules! bad {
    ($val:expr) =>
        println!("{}", $val);  // Missing { }
    ;
}

// Good: proper block
macro_rules! good {
    ($val:expr) => {
        println!("{}", $val);
    };
}
```

---

### Error: "macro expects at least one argument"

```
error: macro expects at least 1 argument
```

**Cause** : Using `+` (one or more) but passing zero arguments.

**Fix** : Use `*` (zero or more) if you want to allow empty calls:

```rust
// With * (zero or more) - allows empty call
macro_rules! printlines {
    ($($line:expr),*) => { ... };  // printlines!() is OK
}

// With + (one or more) - requires at least one
macro_rules! printlines {
    ($($line:expr),+) => { ... };  // printlines!() ERROR!
}
```

---

## Macro Invocation Styles

Macros can be called with different delimiters:

```rust
// All three are equivalent!
printlines!("Hi", 10);      // Parentheses ()
printlines!["Hi", 10];      // Square brackets []
printlines!{"Hi", 10}       // Curly braces {}
```

**Convention** :

- `()` = function-like macros: `println!()`, `printlines!()`
- `[]` = collection macros: `vec![]`
- `{}` = block-like macros: `macro_rules! {}`

---

## Macro Hygiene

### What is Hygiene?

**Hygiene** means macros don't accidentally interfere with surrounding code.

### The Problem (in other languages like C)

```c
// C macro (NOT hygienic)
#define SWAP(a, b) { int temp = a; a = b; b = temp; }

int temp = 100;
int x = 1, y = 2;
SWAP(x, y);
// BUG! The macro's `temp` overwrites our `temp`!
```

### Rust's Solution

```rust
// Rust macro (hygienic!)
macro_rules! swap {
    ($a:expr, $b:expr) => {{
        let temp = $a;  // This `temp` is isolated!
        $a = $b;
        $b = temp;
    }};
}

let temp = 100;  // Our temp
let mut x = 1;
let mut y = 2;
swap!(x, y);     // Works correctly! No conflict!
println!("{}", temp);  // Still 100!
```

**Rust's `macro_rules!` macros are hygienic** - variables defined inside macros don't leak outside!

---

## Performance Considerations

### Macros Have Zero Runtime Cost

```rust
// This macro:
printlines!("Hi", 10);

// Becomes this at compile time:
{
    println!("Value is:");
    println!("{}", "Hi");
    println!();
    println!("Value is:");
    println!("{}", 10);
    println!();
}
```

**There is NO macro at runtime!** It's fully expanded during compilation.

### Compile Time Impact

- More macro calls = slightly longer compile time
- Complex macros = longer compile time
- But **runtime performance is identical** to hand-written code

---

## Summary

### Key Concepts

✅ **Macros** = Code that generates code at compile time

✅ **`macro_rules!`** = Keyword to define declarative macros

✅ **`!`** = Always present when calling a macro

✅ **`$($val:expr),*`** = Match zero or more expressions separated by commas

✅ **`#[macro_export]`** = Export macro to crate root

✅ **Declarative** = Pattern-based (`macro_rules!`)

✅ **Procedural** = Function-based (`#[proc_macro]`)

### Your Macro Explained in One Sentence

`printlines!` takes any number of expressions, and for each one, prints "Value is:" followed by the value - all generated at compile time!

### Quick Reference

```rust
// Define
#[macro_export]
macro_rules! my_macro {
    // No args
    () => { ... };

    // One arg
    ($val:expr) => { ... };

    // Multiple args (comma separated)
    ($($val:expr),*) => { ... };

    // Multiple args with trailing comma
    ($($val:expr),* $(,)?) => { ... };

    // Key-value pairs
    ($($key:expr => $val:expr),*) => { ... };
}

// Use
use MY_CRATE::my_macro;
my_macro!();
my_macro!(1);
my_macro!(1, 2, 3);
```

### Fragment Specifiers Quick Reference

```
expr     → Any expression:        1 + 2, "hello"
ident    → Identifier:            my_var, foo
ty       → Type:                  i32, String
stmt     → Statement:             let x = 5;
item     → Item:                  fn foo() {}, struct Bar
pat      → Pattern:               Some(x), 1..=5
path     → Path:                  std::io::Result
tt       → Token tree:            any single token
block    → Block:                 { ... }
literal  → Literal value:         42, "hi", true
lifetime → Lifetime:              'a, 'static
vis      → Visibility:            pub, pub(crate)
```

### Repetition Markers

```
$( ... )*   → Zero or more times
$( ... )+   → One or more times
$( ... )?   → Zero or one time
```

---

## Additional Resources

### Documentation

- [The Rust Reference - Macros](https://doc.rust-lang.org/reference/macros.html)
- [The Rust Book - Macros](https://doc.rust-lang.org/book/ch19-06-macros.html)
- [Rust by Example - Macros](https://doc.rust-lang.org/rust-by-example/macros.html)

### Crates for Procedural Macros

```toml
syn = "2"           # Parse Rust syntax
quote = "1"         # Generate Rust code
proc-macro2 = "1"   # Token manipulation
thiserror = "1"     # Error derive macro
serde = "1"         # Serialize/Deserialize derive
```
