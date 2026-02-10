
# Rust Modularity & Project Structure - Complete Guide

## Table of Contents

* [Introduction](https://claude.ai/chat/d8a349a5-1109-437b-b4a9-588498bf20cb#introduction)
* [Module System Basics](https://claude.ai/chat/d8a349a5-1109-437b-b4a9-588498bf20cb#module-system-basics)
* [The `mod` Keyword](https://claude.ai/chat/d8a349a5-1109-437b-b4a9-588498bf20cb#the-mod-keyword)
* [The `use` Keyword](https://claude.ai/chat/d8a349a5-1109-437b-b4a9-588498bf20cb#the-use-keyword)
* [Privacy and Visibility](https://claude.ai/chat/d8a349a5-1109-437b-b4a9-588498bf20cb#privacy-and-visibility)
* [File Organization Patterns](https://claude.ai/chat/d8a349a5-1109-437b-b4a9-588498bf20cb#file-organization-patterns)
* [Project Structure Examples](https://claude.ai/chat/d8a349a5-1109-437b-b4a9-588498bf20cb#project-structure-examples)
* [Paths and Imports](https://claude.ai/chat/d8a349a5-1109-437b-b4a9-588498bf20cb#paths-and-imports)
* [Re-exports](https://claude.ai/chat/d8a349a5-1109-437b-b4a9-588498bf20cb#re-exports)
* [Workspaces](https://claude.ai/chat/d8a349a5-1109-437b-b4a9-588498bf20cb#workspaces)
* [Best Practices](https://claude.ai/chat/d8a349a5-1109-437b-b4a9-588498bf20cb#best-practices)
* [Common Patterns](https://claude.ai/chat/d8a349a5-1109-437b-b4a9-588498bf20cb#common-patterns)
* [Troubleshooting](https://claude.ai/chat/d8a349a5-1109-437b-b4a9-588498bf20cb#troubleshooting)

---

## Introduction

Rust's module system helps you organize code into logical units, manage namespaces, and control privacy. Understanding modules is crucial for building scalable, maintainable Rust applications.

### Key Components

* **`mod`** - Declares modules
* **`use`** - Brings items into scope
* **`pub`** - Makes items public
* **`crate`** - Root of the module tree
* **`super`** - Parent module
* **`self`** - Current module

---

## Module System Basics

### What is a Module?

A module is a namespace that contains definitions (functions, structs, traits, etc.). Modules help you:

* **Organize code** into logical units
* **Control visibility** (public vs private)
* **Prevent naming conflicts**
* **Create clear APIs**

### Module Hierarchy

```
crate (root)
  ├── module_a
  │   ├── submodule_1
  │   └── submodule_2
  └── module_b
      └── submodule_3
```

---

## The `mod` Keyword

### Inline Modules

Defined directly in the file:

```rust
// main.rs
mod math {
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }
  
    pub fn subtract(a: i32, b: i32) -> i32 {
        a - b
    }
}

fn main() {
    let result = math::add(5, 3);
    println!("Result: {}", result);
}
```

### Nested Modules

```rust
mod outer {
    pub mod inner {
        pub fn greet() {
            println!("Hello from inner!");
        }
    }
}

fn main() {
    outer::inner::greet();
}
```

### Modules in Separate Files

**File: `main.rs`**

```rust
mod math;  // Tells Rust to look for math.rs

fn main() {
    let result = math::add(10, 5);
    println!("10 + 5 = {}", result);
}
```

**File: `math.rs`**

```rust
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn subtract(a: i32, b: i32) -> i32 {
    a - b
}
```

### Modules as Directories

**Structure:**

```
src/
├── main.rs
└── math/
    ├── mod.rs       # Module root
    ├── basic.rs     # Submodule
    └── advanced.rs  # Submodule
```

**File: `main.rs`**

```rust
mod math;

fn main() {
    math::basic::add(5, 3);
    math::advanced::power(2, 3);
}
```

**File: `math/mod.rs`**

```rust
pub mod basic;
pub mod advanced;
```

**File: `math/basic.rs`**

```rust
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn subtract(a: i32, b: i32) -> i32 {
    a - b
}
```

**File: `math/advanced.rs`**

```rust
pub fn power(base: i32, exp: u32) -> i32 {
    base.pow(exp)
}

pub fn factorial(n: u32) -> u32 {
    (1..=n).product()
}
```

---

## The `use` Keyword

The `use` keyword brings items into scope, avoiding repetitive paths.

### Basic Usage

```rust
mod math {
    pub fn add(a: i32, b: i32) -> i32 { a + b }
}

use math::add;  // Bring `add` into scope

fn main() {
    let result = add(5, 3);  // No need for math::add
}
```

### Multiple Items

```rust
use std::io::{self, Write, Read};  // Import multiple items
use std::collections::{HashMap, HashSet};
```

### Renaming with `as`

```rust
use std::fmt::Result;
use std::io::Result as IoResult;  // Avoid naming conflict

fn function1() -> Result { Ok(()) }
fn function2() -> IoResult<()> { Ok(()) }
```

### Glob Imports

```rust
use std::collections::*;  // Import everything (use sparingly!)

fn main() {
    let map = HashMap::new();
    let set = HashSet::new();
}
```

⚠️  **Warning** : Glob imports can make code less readable.

### Re-exporting with `pub use`

```rust
// lib.rs
mod internal {
    pub fn helper() {
        println!("Internal helper");
    }
}

pub use internal::helper;  // Re-export for external use
```

Now users can call `your_crate::helper()` instead of `your_crate::internal::helper()`.

---

## Privacy and Visibility

### Default: Private

Everything in Rust is  **private by default** .

```rust
mod my_module {
    fn private_function() {  // Private
        println!("Can't call from outside");
    }
  
    pub fn public_function() {  // Public
        println!("Can call from anywhere");
        private_function();  // OK within module
    }
}

fn main() {
    // my_module::private_function();  // ERROR!
    my_module::public_function();      // OK
}
```

### Visibility Modifiers

| Modifier         | Visibility                |
| ---------------- | ------------------------- |
| *(none)*       | Private to current module |
| `pub`          | Public to all             |
| `pub(crate)`   | Public within crate only  |
| `pub(super)`   | Public to parent module   |
| `pub(in path)` | Public to specific path   |

#### Examples

```rust
mod outer {
    pub(crate) fn crate_visible() {}  // Visible in entire crate
    pub(super) fn parent_visible() {} // Visible to parent only
  
    pub mod inner {
        pub(in crate::outer) fn outer_visible() {}  // Visible in outer
    }
}
```

### Struct Field Visibility

```rust
pub struct User {
    pub username: String,      // Public field
    email: String,             // Private field
}

impl User {
    pub fn new(username: String, email: String) -> Self {
        User { username, email }
    }
  
    pub fn get_email(&self) -> &str {
        &self.email
    }
}
```

### Enum Variants

If an enum is `pub`, all its variants are automatically public:

```rust
pub enum Status {
    Active,    // Automatically public
    Inactive,  // Automatically public
}
```

---

## File Organization Patterns

### Pattern 1: Flat Structure (Small Projects)

```
src/
├── main.rs
├── lib.rs
├── utils.rs
└── config.rs
```

**main.rs:**

```rust
mod utils;
mod config;

fn main() {
    utils::helper();
    config::load();
}
```

### Pattern 2: Module Directories (Medium Projects)

```
src/
├── main.rs
├── database/
│   ├── mod.rs
│   ├── connection.rs
│   └── queries.rs
├── api/
│   ├── mod.rs
│   ├── routes.rs
│   └── handlers.rs
└── models/
    ├── mod.rs
    ├── user.rs
    └── post.rs
```

**main.rs:**

```rust
mod database;
mod api;
mod models;

fn main() {
    database::connection::connect();
    api::routes::setup();
}
```

**database/mod.rs:**

```rust
pub mod connection;
pub mod queries;
```

### Pattern 3: Feature-Based (Large Projects)

```
src/
├── main.rs
├── lib.rs
├── features/
│   ├── mod.rs
│   ├── authentication/
│   │   ├── mod.rs
│   │   ├── login.rs
│   │   ├── signup.rs
│   │   └── session.rs
│   ├── user_management/
│   │   ├── mod.rs
│   │   ├── profile.rs
│   │   └── settings.rs
│   └── payments/
│       ├── mod.rs
│       ├── checkout.rs
│       └── refund.rs
└── shared/
    ├── mod.rs
    ├── utils.rs
    └── constants.rs
```

**features/mod.rs:**

```rust
pub mod authentication;
pub mod user_management;
pub mod payments;
```

**lib.rs:**

```rust
pub mod features;
pub mod shared;
```

---

## Project Structure Examples

### Example 1: CLI Application

```
my_cli_app/
├── Cargo.toml
├── src/
│   ├── main.rs
│   ├── cli/
│   │   ├── mod.rs
│   │   ├── args.rs
│   │   └── commands.rs
│   ├── core/
│   │   ├── mod.rs
│   │   ├── processor.rs
│   │   └── validator.rs
│   └── utils/
│       ├── mod.rs
│       ├── file.rs
│       └── logger.rs
└── tests/
    └── integration_tests.rs
```

**main.rs:**

```rust
mod cli;
mod core;
mod utils;

use cli::args::parse_args;
use core::processor::process;

fn main() {
    let args = parse_args();
    process(args);
}
```

**cli/mod.rs:**

```rust
pub mod args;
pub mod commands;
```

**cli/args.rs:**

```rust
pub fn parse_args() -> Args {
    // Argument parsing logic
}

pub struct Args {
    pub input: String,
    pub output: String,
}
```

### Example 2: Web API

```
web_api/
├── Cargo.toml
├── src/
│   ├── main.rs
│   ├── lib.rs
│   ├── api/
│   │   ├── mod.rs
│   │   ├── routes.rs
│   │   ├── handlers.rs
│   │   └── middleware.rs
│   ├── database/
│   │   ├── mod.rs
│   │   ├── connection.rs
│   │   └── models.rs
│   ├── services/
│   │   ├── mod.rs
│   │   ├── user_service.rs
│   │   └── auth_service.rs
│   └── config/
│       ├── mod.rs
│       └── settings.rs
├── tests/
└── migrations/
```

**lib.rs:**

```rust
pub mod api;
pub mod database;
pub mod services;
pub mod config;
```

**main.rs:**

```rust
use web_api::{api, config, database};

#[tokio::main]
async fn main() {
    let config = config::settings::load();
    database::connection::init(&config).await;
    api::routes::start_server(config).await;
}
```

### Example 3: Library Crate

```
my_library/
├── Cargo.toml
├── src/
│   ├── lib.rs
│   ├── core/
│   │   ├── mod.rs
│   │   ├── engine.rs
│   │   └── processor.rs
│   ├── utils/
│   │   ├── mod.rs
│   │   └── helpers.rs
│   └── prelude.rs
├── examples/
│   └── basic_usage.rs
└── tests/
    └── integration.rs
```

**lib.rs:**

```rust
pub mod core;
pub mod utils;
pub mod prelude;

// Re-export commonly used items
pub use core::engine::Engine;
pub use core::processor::Processor;
```

**prelude.rs:**

```rust
// Convenience module for glob imports
pub use crate::core::engine::*;
pub use crate::core::processor::*;
pub use crate::utils::helpers::*;
```

**Usage by consumers:**

```rust
use my_library::prelude::*;

fn main() {
    let engine = Engine::new();
}
```

---

## Paths and Imports

### Absolute Paths (from crate root)

```rust
use crate::database::models::User;  // Starts from crate root
```

### Relative Paths

```rust
use self::internal::helper;   // Current module
use super::parent_function;   // Parent module
use super::super::grandparent_function;  // Grandparent module
```

### Example Structure

```
src/
├── main.rs
└── networking/
    ├── mod.rs
    ├── client.rs
    └── server.rs
```

**networking/client.rs:**

```rust
// Absolute path
use crate::networking::server::ServerConfig;

// Relative path
use super::server::ServerConfig;

pub struct Client {
    config: ServerConfig,
}
```

### External Crates

```rust
// From Cargo.toml dependencies
use serde::{Serialize, Deserialize};
use tokio::runtime::Runtime;
use std::collections::HashMap;  // Standard library
```

---

## Re-exports

Re-exporting simplifies APIs and hides internal structure.

### Internal Structure

```
src/
├── lib.rs
└── internal/
    ├── mod.rs
    ├── parser.rs
    └── formatter.rs
```

**internal/parser.rs:**

```rust
pub struct Parser;
impl Parser {
    pub fn parse(input: &str) -> Result<(), String> {
        Ok(())
    }
}
```

**lib.rs:**

```rust
mod internal;

// Re-export for clean public API
pub use internal::parser::Parser;
pub use internal::formatter::Formatter;
```

**User code:**

```rust
use my_crate::Parser;  // Clean, doesn't expose internal structure

fn main() {
    let parser = Parser;
}
```

### Flattening Module Hierarchy

```rust
// lib.rs
mod deeply {
    pub mod nested {
        pub mod module {
            pub fn important_function() {}
        }
    }
}

// Flatten for users
pub use deeply::nested::module::important_function;
```

**Users can now:**

```rust
use my_crate::important_function;  // Instead of deeply::nested::module::
```

---

## Workspaces

Workspaces allow managing multiple related packages in one repository.

### Workspace Structure

```
my_workspace/
├── Cargo.toml          # Workspace manifest
├── app/
│   ├── Cargo.toml
│   └── src/
│       └── main.rs
├── lib_core/
│   ├── Cargo.toml
│   └── src/
│       └── lib.rs
└── lib_utils/
    ├── Cargo.toml
    └── src/
        └── lib.rs
```

**Root Cargo.toml:**

```toml
[workspace]
members = [
    "app",
    "lib_core",
    "lib_utils",
]

[workspace.dependencies]
serde = "1.0"
```

**app/Cargo.toml:**

```toml
[package]
name = "app"
version = "0.1.0"
edition = "2021"

[dependencies]
lib_core = { path = "../lib_core" }
lib_utils = { path = "../lib_utils" }
serde = { workspace = true }
```

**app/src/main.rs:**

```rust
use lib_core::Engine;
use lib_utils::helpers;

fn main() {
    let engine = Engine::new();
    helpers::initialize();
}
```

### Workspace Benefits

✅ **Shared dependencies** - Single `Cargo.lock`

✅ **Unified builds** - `cargo build` builds all packages

✅ **Code sharing** - Easy inter-package dependencies

✅ **Version consistency** - Shared dependency versions

---

## Best Practices

### 1. Keep Modules Focused

Each module should have a  **single responsibility** .

❌ **Bad:**

```rust
mod utils {
    pub fn http_request() {}
    pub fn database_query() {}
    pub fn file_operation() {}
}
```

✅ **Good:**

```rust
mod http {
    pub fn request() {}
}

mod database {
    pub fn query() {}
}

mod filesystem {
    pub fn read_file() {}
}
```

### 2. Use Clear Names

Module names should clearly describe their purpose.

✅ **Good names:**

* `authentication`
* `database`
* `api_handlers`
* `user_models`

❌ **Bad names:**

* `stuff`
* `misc`
* `helpers` (too vague)
* `utils` (what kind?)

### 3. Expose Minimal Public API

Only make public what's necessary.

```rust
// lib.rs
mod internal {
    pub(crate) fn internal_helper() {}  // Only visible in crate
}

pub mod api {
    pub fn public_function() {
        crate::internal::internal_helper();
    }
}
```

### 4. Use `mod.rs` vs Module Files

 **Modern Rust (2018+)** : Prefer named files

```
src/
├── database.rs      # Better
└── database/
    └── mod.rs       # Old style (still valid)
```

 **Exception** : Use `mod.rs` when you have multiple submodules.

### 5. Group Related Items

```rust
// models.rs
pub mod user;
pub mod post;
pub mod comment;

// Re-export for convenience
pub use user::User;
pub use post::Post;
pub use comment::Comment;
```

### 6. Avoid Circular Dependencies

❌ **Bad:**

```rust
// module_a.rs
use crate::module_b::FunctionB;

// module_b.rs
use crate::module_a::FunctionA;  // Circular!
```

✅ **Solution:** Extract shared code to a third module.

```rust
// shared.rs
pub struct SharedData;

// module_a.rs
use crate::shared::SharedData;

// module_b.rs
use crate::shared::SharedData;
```

### 7. Use Prelude for Common Imports

```rust
// prelude.rs
pub use crate::errors::*;
pub use crate::types::*;
pub use crate::traits::*;

// Users can import everything at once
use my_crate::prelude::*;
```

---

## Common Patterns

### Pattern 1: Error Module

```rust
// errors.rs
use std::fmt;

#[derive(Debug)]
pub enum AppError {
    NotFound,
    Unauthorized,
    Internal(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::NotFound => write!(f, "Resource not found"),
            AppError::Unauthorized => write!(f, "Unauthorized access"),
            AppError::Internal(msg) => write!(f, "Internal error: {}", msg),
        }
    }
}

impl std::error::Error for AppError {}
```

### Pattern 2: Types Module

```rust
// types.rs
use std::collections::HashMap;

pub type UserId = u64;
pub type Username = String;
pub type UserMap = HashMap<UserId, User>;

pub struct User {
    pub id: UserId,
    pub name: Username,
}
```

### Pattern 3: Constants Module

```rust
// constants.rs
pub const MAX_CONNECTIONS: usize = 100;
pub const TIMEOUT_SECONDS: u64 = 30;
pub const API_VERSION: &str = "v1";
```

### Pattern 4: Config Module

```rust
// config/mod.rs
pub mod database;
pub mod server;
pub mod logging;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub database: database::DatabaseConfig,
    pub server: server::ServerConfig,
}

impl Config {
    pub fn load() -> Result<Self, ConfigError> {
        // Load from file or environment
    }
}
```

### Pattern 5: Traits Module

```rust
// traits.rs
pub trait Authenticatable {
    fn authenticate(&self, credentials: &Credentials) -> bool;
}

pub trait Authorizable {
    fn authorize(&self, permission: &Permission) -> bool;
}

pub trait Loggable {
    fn log(&self);
}
```

---

## Troubleshooting

### Error: "unresolved import"

**Problem:**

```rust
use crate::my_module::MyStruct;  // Error!
```

**Solutions:**

1. **Check module declaration:**
   ```rust
   mod my_module;  // Add this to parent module
   ```
2. **Check visibility:**
   ```rust
   pub struct MyStruct;  // Make it public
   ```
3. **Check file location:**
   * `my_module.rs` should be in same directory, or
   * `my_module/mod.rs` should exist

### Error: "cannot find value in this scope"

**Problem:**

```rust
fn main() {
    helper_function();  // Error: not found
}
```

**Solutions:**

1. **Use full path:**
   ```rust
   my_module::helper_function();
   ```
2. **Bring into scope:**
   ```rust
   use my_module::helper_function;

   fn main() {
       helper_function();
   }
   ```

### Error: "private module"

**Problem:**

```rust
// lib.rs
mod internal;  // Private!

// main.rs
use my_crate::internal::function;  // Error!
```

**Solution:**

```rust
// lib.rs
pub mod internal;  // Make public
```

### Circular Dependency Errors

**Problem:**

```
error[E0369]: binary operation `==` cannot be applied to type `A`
```

**Solution:** Extract shared types to separate module.

---

## Summary

### Quick Reference

| Keyword        | Purpose          | Example                         |
| -------------- | ---------------- | ------------------------------- |
| `mod`        | Declare module   | `mod my_module;`              |
| `use`        | Bring into scope | `use std::io::Read;`          |
| `pub`        | Make public      | `pub fn hello() {}`           |
| `pub(crate)` | Public in crate  | `pub(crate) struct Internal;` |
| `pub(super)` | Public to parent | `pub(super) fn helper() {}`   |
| `crate::`    | Absolute path    | `use crate::models::User;`    |
| `super::`    | Parent module    | `use super::parent_fn;`       |
| `self::`     | Current module   | `use self::internal;`         |

### Module Declaration Checklist

* [ ] Module declared with `mod`
* [ ] Module file exists in correct location
* [ ] Items marked `pub` as needed
* [ ] Used `use` to bring items into scope
* [ ] No circular dependencies
* [ ] Clear, descriptive names
* [ ] Appropriate privacy levels

---

## Further Reading

* [The Rust Book - Modules](https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html)
* [Rust By Example - Modules](https://doc.rust-lang.org/rust-by-example/mod.html)
* [Cargo Book - Workspaces](https://doc.rust-lang.org/cargo/reference/workspaces.html)
* [API Guidelines](https://rust-lang.github.io/api-guidelines/)
