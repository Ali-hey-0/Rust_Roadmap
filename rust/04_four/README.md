# README for Rust Code Analysis

This `README.md` documents the structure, purpose, and Rust features utilized in the provided source file.

## Overview

The provided Rust code defines several custom data types (`enum` and `struct`) and demonstrates basic pattern matching using `match` and `if let`. A key focus of the original logic was managing Rust's **move semantics** when using the same data structure across multiple checks, which was resolved using `.clone()`.

---

## Data Structures Defined

### 1. Enums

A simple enum representing 2D or 3D points, using tuple variants.

```rust
enum Point{
    D2(i32,i32), 
    D3(i32,i32,i32) 
}
```

* **Variants:** `D2` (two `i32`s) and `D3` (three `i32`s).
* **Derives:** `#[derive(Debug, Clone)]` is used. `Debug` for easy printing (though not used extensively here), and `Clone` to allow making copies of the enum instance.

A simple C-style enum with no associated data.

```rust
enum Gender{
    Male,Female
}
```

A complex enum representing HTTP status codes, utilizing both tuple and struct variants.

```rust
#[derive(Debug, Clone)] 
enum HTTPStatus{
    Info{code:u8,desc:String},       // Struct variant (named fields)
    Ok(u8,String),                  // Tuple variant
    Redirect(u8,String),            // Tuple variant (the focus of pattern matching tests)
    ClientError(u8,String), 
    ServerError(u8,String), 
}
```

* **Variants:** Uses both named field variants (`Info`) and tuple variants (`Ok`, `Redirect`, etc.).
* **Derives:** `Clone` is essential here to allow the `b` variable to be checked multiple times without ownership issues.

An enum demonstrating various ways to store color data using tuple variants (RGB, BGR, RGBA, CMYK, and Gray).

---

### 2. Structs

A simple tuple struct.

```rust
struct P(i32,i32); 
```

A unit struct (no fields).

```rust
struct Test;
```

A standard struct with two integer fields.

```rust
#[derive(Default)]
struct Test1{
    v1:i32,
    v2:i32
}
```

* **Derives:** `Default` allows initializing `Test1` using `Test1::default()`, which initializes fields to their default values (`v1: 0`, `v2: 0`).

---

## Core Logic in Function

The `main` function focuses on pattern matching and ownership management.

### 1. Matching

A simple demonstration of pattern matching on the `Point` enum:

```rust
let a : Point = Point::D2(10,-1);

match a {
    Point::D2(x,y) => println!("{},{}",x ,y), // Destructures the tuple variant
    Point::D3(_,_,_) => (),
};
// Output: 10,-1
```

### 2. Matching and Ownership Management

This section was the primary area of iteration:

1. **Initialization:** A single instance of `b` is created as `HTTPStatus::Ok(200, "OK")`.
2. **Move Semantics Handling:** Since the original variable `b` cannot be used after being moved (consumed) by a pattern match, we must use `b.clone()` for every subsequent check that requires ownership of the data inside. This is possible because `HTTPStatus` derives `Clone`.
3. **Pattern Matching Fix (E0769 Resolution):** The error regarding pattern syntax for tuple variants was fixed.
   * **Incorrect (Struct syntax for Tuple variant):** `HTTPStatus::Redirect{code, desc}`
   * **Correct (Tuple syntax):** `HTTPStatus::Redirect(code, desc)`

The final successful checks are:

```rust
// Check 1: Match statement
match b.clone() { 
    HTTPStatus::Redirect(code, _) => println!("Following from match: {}", code),
    _ => ()
};

// Check 2: If Let statement (Corrected Tuple Pattern)
if let HTTPStatus::Redirect(code, desc) = b.clone() { 
    println!("Following from if let: Code {} Desc {}", code, desc);
}
```

Since `b` is initialized as `Ok`, **neither** the `match` nor the `if let` block targeting `Redirect` will execute. The output only shows the result from the `Point` match.
