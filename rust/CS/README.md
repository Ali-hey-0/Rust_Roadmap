# Rust Cross-Compilation & Toolchains
## Complete Technical Guide

---

## Table of Contents

| # | Section | Core Concept |
|---|---------|--------------|
| 1 | [Compilers — GCC vs Clang vs MSVC](#1-compilers--gcc-vs-clang-vs-msvc) | What they are, how they differ |
| 2 | [MinGW — Windows + GCC](#2-mingw--windows--gcc) | GNU toolchain on Windows |
| 3 | [Toolchains](#3-toolchains) | Complete compiler + linker + stdlib stack |
| 4 | [Rust Target Triples](#4-rust-target-triples) | Anatomy of `x86_64-pc-windows-gnu` |
| 5 | [Rustup — Toolchain Manager](#5-rustup--toolchain-manager) | Installing/managing Rust toolchains |
| 6 | [Installing Targets](#6-installing-targets) | `rustup target add` workflow |
| 7 | [Cargo Build with Targets](#7-cargo-build-with-targets) | `--target` flag mechanics |
| 8 | [Cross — Docker-Based Cross-Compiler](#8-cross--docker-based-cross-compiler) | How it works, when to use it |
| 9 | [Complete Examples](#9-complete-examples) | Windows → Linux, macOS → Windows, etc. |
| 10 | [Troubleshooting](#10-troubleshooting) | Common errors and fixes |

---

## 1. Compilers — GCC vs Clang vs MSVC

### 1.1 What Is a Compiler?

A **compiler** translates source code (C, C++, Rust) into machine code (binary executable).

```
Source Code  ──→  [COMPILER]  ──→  Machine Code
  .c, .rs              ↓              .exe, .dll
                  (many stages)
```

The process actually involves multiple stages:

```
┌─────────────────────────────────────────────────────────┐
│  1. Preprocessing  →  Expand macros, includes           │
│  2. Compilation    →  Source → Assembly                 │
│  3. Assembly       →  Assembly → Object code (.o)       │
│  4. Linking        →  Object files → Executable         │
└─────────────────────────────────────────────────────────┘
```

### 1.2 GCC (GNU Compiler Collection)

**Origin:** GNU Project (1987), created to be a free Unix compiler  
**Languages:** C, C++, Objective-C, Fortran, Ada, Go, D  
**Platforms:** Linux (default), macOS (legacy), Windows (via MinGW)

**Key characteristics:**
- **Open source** (GPL license)
- **Battle-tested** — 35+ years of optimization
- **Default on Linux** — most Linux binaries compiled with GCC
- **Standards-compliant** — excellent C/C++ standard support

**Command-line tools:**
```bash
gcc       # C compiler
g++       # C++ compiler
ld        # GNU linker
ar        # Archive utility (creates .a libraries)
```

**Example:**
```bash
gcc -o program program.c      # Compile C to executable
gcc -c file.c                 # Compile to object file (.o)
gcc file1.o file2.o -o prog   # Link object files
```

### 1.3 Clang/LLVM

**Origin:** University of Illinois (2007), now maintained by Apple  
**Languages:** C, C++, Objective-C, Swift, Rust (via LLVM backend)  
**Platforms:** macOS (default since Xcode 5), Linux, Windows

**Key characteristics:**
- **Modular architecture** — compiler frontend + LLVM backend
- **Better error messages** — more human-readable than GCC
- **Faster compilation** (in many cases)
- **Apple's preferred compiler** — optimized for macOS/iOS

**Architecture:**
```
Source Code  ──→  [Clang Frontend]  ──→  LLVM IR  ──→  [LLVM Backend]  ──→  Machine Code
                  (parsing, analysis)    (intermediate)  (optimization)
```

**Command-line tools:**
```bash
clang      # C compiler
clang++    # C++ compiler
lld        # LLVM linker (fast alternative to GNU ld)
llvm-ar    # Archive utility
```

**Why Rust uses LLVM:**
- Rust compiler (`rustc`) generates **LLVM IR**, then LLVM optimizes and generates machine code
- This allows Rust to benefit from LLVM's mature optimization passes
- Rust doesn't use Clang, but shares LLVM's backend

```
rustc  ──→  LLVM IR  ──→  [LLVM]  ──→  Native code
            (MIR → LLVM)  (opt)        (.exe, .so)
```

### 1.4 MSVC (Microsoft Visual C++)

**Origin:** Microsoft (1983)  
**Languages:** C, C++  
**Platform:** Windows only

**Key characteristics:**
- **Closed source** (proprietary)
- **Default on Windows** — Visual Studio ships with MSVC
- **Best Windows integration** — native support for Windows APIs, DLLs, COM
- **PDB debug format** — Visual Studio debugger uses this
- **C++ ABI incompatibility with GCC/Clang** — can't mix MSVC and GCC object files

**Command-line tools:**
```cmd
cl.exe       # Compiler
link.exe     # Linker
lib.exe      # Library manager
nmake.exe    # Build tool (like make)
```

**Example:**
```cmd
cl /Fe:program.exe program.c    # Compile to executable
cl /c file.c                    # Compile to object file (.obj)
link file1.obj file2.obj        # Link
```

### 1.5 Comparison Table

| Feature | GCC | Clang | MSVC |
|---------|-----|-------|------|
| **License** | GPL (free) | Apache 2.0 (free) | Proprietary |
| **Default OS** | Linux | macOS | Windows |
| **C++ ABI** | Itanium ABI | Itanium ABI | Microsoft ABI |
| **Error messages** | Terse | Excellent | Good |
| **Compile speed** | Medium | Fast | Medium |
| **Optimization** | Excellent | Excellent | Excellent |
| **Debug format** | DWARF | DWARF | PDB |
| **Rust support** | Via GCC linker | Via LLVM backend | Via linker |

### 1.6 Why Multiple Compilers Matter for Rust

**Rust doesn't directly use GCC/Clang to compile Rust code.** Instead:

1. **`rustc` compiles Rust → LLVM IR**
2. **LLVM backend generates machine code**
3. **System linker links everything together**

The **linker** is where GCC/Clang/MSVC come in:

```
┌───────────────────────────────────────────────────────────┐
│  Rust crate (your code)                                   │
│     ↓                                                      │
│  rustc  ──→  LLVM  ──→  .o / .obj files                   │
│     ↓                                                      │
│  System linker (ld, lld, or link.exe)                     │
│     ↓                                                      │
│  Links against:                                            │
│    • libc (glibc, musl, msvcrt.dll)                       │
│    • System libraries (pthread, WinAPI)                   │
│    • C dependencies (if you use FFI)                      │
│     ↓                                                      │
│  Final executable                                          │
└───────────────────────────────────────────────────────────┘
```

**Which linker gets used depends on your target triple.**

---

## 2. MinGW — Windows + GCC

### 2.1 What Is MinGW?

**MinGW = Minimalist GNU for Windows**

A port of the GNU toolchain (GCC, binutils) to Windows. Allows you to compile native Windows programs using GCC instead of MSVC.

```
┌──────────────────────────────────────────────────────────┐
│  MinGW = GCC compiler + GNU linker + Windows headers     │
│                                                          │
│  Produces:  .exe and .dll files                          │
│  Links against:  msvcrt.dll (Microsoft C runtime)        │
│  Does NOT require:  Visual Studio                        │
└──────────────────────────────────────────────────────────┘
```

### 2.2 MinGW vs MinGW-w64

| Name | Description |
|------|-------------|
| **MinGW** (original) | 32-bit only, limited Windows API support, unmaintained |
| **MinGW-w64** | Both 32-bit and 64-bit, complete Windows API headers, actively maintained |

**Everyone uses MinGW-w64 now.** When you see "MinGW" in 2025, it almost always means MinGW-w64.

### 2.3 Why MinGW Exists

**Problem:** GCC was designed for Unix. Windows uses a different:
- API (WinAPI instead of POSIX)
- C runtime (msvcrt.dll instead of glibc)
- Object file format (PE/COFF instead of ELF)
- Calling conventions (stdcall, fastcall)

**MinGW's solution:**
- Port GCC to understand PE/COFF object files
- Provide Windows-specific headers (`windows.h`, `winsock2.h`)
- Link against Windows system DLLs

### 2.4 MinGW Toolchain Components

```
┌──────────────────────────────────────────────────────┐
│  x86_64-w64-mingw32-gcc      ← GCC compiler          │
│  x86_64-w64-mingw32-g++      ← G++ compiler          │
│  x86_64-w64-mingw32-ld       ← GNU linker            │
│  x86_64-w64-mingw32-ar       ← Archive tool          │
│  x86_64-w64-mingw32-objdump  ← Object file inspector │
│  x86_64-w64-mingw32-strip    ← Symbol stripper      │
└──────────────────────────────────────────────────────┘
```

The `x86_64-w64-mingw32-` prefix indicates:
- `x86_64`: 64-bit architecture
- `w64`: MinGW-w64 project
- `mingw32`: Target is Windows (legacy name, works for 64-bit too)

### 2.5 Installing MinGW on Windows

**Option 1: MSYS2 (recommended)**
```bash
# Install MSYS2 from https://www.msys2.org/
# Then install MinGW toolchain:
pacman -S mingw-w64-x86_64-gcc
```

**Option 2: Standalone installer**
```
Download from: https://github.com/niXman/mingw-builds-binaries/releases
Extract to C:\mingw64
Add C:\mingw64\bin to PATH
```

**Option 3: Install via Rust**
```bash
rustup toolchain install stable-x86_64-pc-windows-gnu
# This installs a bundled MinGW
```

### 2.6 MSVC vs MinGW for Rust

| Aspect | MSVC (`-pc-windows-msvc`) | MinGW (`-pc-windows-gnu`) |
|--------|---------------------------|---------------------------|
| **C runtime** | msvcrt.dll (dynamic) or static | msvcrt.dll (dynamic) |
| **C++ ABI** | Microsoft ABI | Itanium ABI |
| **Debugger** | Visual Studio (PDB) | GDB (DWARF) |
| **Binary size** | Smaller (dynamic linking) | Larger (static linking) |
| **Dependencies** | Requires MSVC redistributable | Minimal dependencies |
| **Performance** | Slightly better on Windows | Comparable |
| **Cross-compile from Linux** | ❌ Not possible | ✅ Possible |

**When to use each:**

```
Use MSVC when:
  • Developing on Windows with Visual Studio
  • Need best Windows tooling integration
  • Building Windows-only software

Use MinGW when:
  • Cross-compiling from Linux/macOS to Windows
  • Need GCC-specific features
  • Want to avoid MSVC redistributable dependencies
  • Building open-source projects with Unix roots
```

---

## 3. Toolchains

### 3.1 What Is a Toolchain?

A **toolchain** is a complete set of tools needed to build software for a specific target platform:

```
┌──────────────────────────────────────────────────────┐
│  Toolchain = Compiler + Linker + Standard Library    │
│              + Headers + Build Tools                  │
└──────────────────────────────────────────────────────┘
```

**Full toolchain example (Linux → ARM embedded):**
```
arm-none-eabi-gcc        ← Compiler
arm-none-eabi-ld         ← Linker
arm-none-eabi-ar         ← Archiver
arm-none-eabi-objcopy    ← Binary utility
libc.a                   ← C standard library
libgcc.a                 ← Compiler runtime
crt0.o                   ← Startup code
```

### 3.2 Native vs Cross Toolchain

**Native toolchain:** Builds for the **same** platform you're running on

```
Linux x86_64  ──→  [GCC]  ──→  Linux x86_64 binary
     ↑                              ↑
   (host)                        (target)
     └──────── SAME ──────────────┘
```

**Cross toolchain:** Builds for a **different** platform

```
Linux x86_64  ──→  [ARM GCC]  ──→  ARM Linux binary
     ↑                                  ↑
   (host)                            (target)
     └──────── DIFFERENT ────────────┘
```

### 3.3 Rust Toolchains

Rust has **multiple toolchains** installed via `rustup`:

```bash
rustup toolchain list
# Output:
# stable-x86_64-unknown-linux-gnu (default)
# nightly-x86_64-unknown-linux-gnu
# 1.75.0-x86_64-unknown-linux-gnu
```

Each toolchain includes:
```
┌────────────────────────────────────────────────────┐
│  rustc              ← Rust compiler                │
│  cargo              ← Build tool                   │
│  rust-std           ← Standard library             │
│  rustfmt            ← Code formatter               │
│  clippy             ← Linter                       │
└────────────────────────────────────────────────────┘
```

### 3.4 Toolchain Directory Structure

```
~/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/
├── bin/
│   ├── rustc          ← Compiler
│   ├── cargo          ← Build tool
│   └── rust-gdb       ← Debugger wrapper
├── lib/
│   ├── librustc_driver.so
│   └── rustlib/
│       └── x86_64-unknown-linux-gnu/
│           ├── lib/
│           │   ├── libstd.rlib      ← Standard library
│           │   └── libcore.rlib     ← Core library
│           └── bin/
│               └── rust-lld         ← LLVM linker
└── share/
    └── doc/           ← Documentation
```

---

## 4. Rust Target Triples

### 4.1 Anatomy of a Target Triple

Rust uses **target triples** to specify compilation targets. Format:

```
<architecture>-<vendor>-<os>-<environment>
     ↓            ↓       ↓         ↓
  x86_64    -    pc   - windows  - gnu
```

### 4.2 Field Meanings

**Architecture:**
```
x86_64        → 64-bit x86 (Intel/AMD)
i686          → 32-bit x86
aarch64       → 64-bit ARM (Apple Silicon, Raspberry Pi 4+)
armv7         → 32-bit ARM
wasm32        → WebAssembly
riscv64       → RISC-V 64-bit
```

**Vendor:**
```
pc            → Generic PC
apple         → Apple (macOS, iOS)
unknown       → Unknown/unspecified vendor
w64           → MinGW-w64
```

**OS:**
```
linux         → Linux
windows       → Windows
darwin        → macOS
android       → Android
none          → Bare metal (no OS)
```

**Environment:**
```
gnu           → GNU toolchain (GCC, glibc)
musl          → Musl libc (static linking)
msvc          → Microsoft Visual C++
eabi          → Embedded ABI
```

### 4.3 Common Rust Targets

| Target Triple | Description |
|---------------|-------------|
| `x86_64-unknown-linux-gnu` | Linux 64-bit (GCC, glibc, dynamic linking) |
| `x86_64-unknown-linux-musl` | Linux 64-bit (musl libc, static linking) |
| `x86_64-pc-windows-msvc` | Windows 64-bit (MSVC toolchain) |
| `x86_64-pc-windows-gnu` | Windows 64-bit (MinGW/GCC toolchain) |
| `x86_64-apple-darwin` | macOS Intel 64-bit |
| `aarch64-apple-darwin` | macOS Apple Silicon (M1/M2/M3) |
| `aarch64-unknown-linux-gnu` | Linux ARM 64-bit (Raspberry Pi, AWS Graviton) |
| `wasm32-unknown-unknown` | WebAssembly (browser, WASI) |
| `thumbv7em-none-eabihf` | ARM Cortex-M4 embedded (no OS, hard float) |

### 4.4 Detailed Example: `x86_64-pc-windows-gnu`

```
x86_64  -  pc  -  windows  -  gnu
  ↓        ↓        ↓          ↓
 64-bit   Generic  Windows   GNU tools
  Intel    PC                (MinGW, GCC linker)
```

**What this means:**
- **CPU:** 64-bit x86 (Intel/AMD processors)
- **Platform:** Standard PC hardware
- **Operating System:** Windows 10/11
- **Toolchain:** GNU-based (MinGW-w64)
  - Linker: `ld.exe` (GNU linker)
  - C runtime: `msvcrt.dll` (Windows C runtime)
  - Standard library: `libstd-<hash>.dll`

**Contrast with MSVC:**
```
x86_64  -  pc  -  windows  -  msvc
                               ↑
                          Microsoft toolchain
                          (link.exe, PDB debugging)
```

### 4.5 Finding All Available Targets

```bash
rustc --print target-list | wc -l
# Output: 200+

# Filter for Windows targets:
rustc --print target-list | grep windows
# Output:
# i686-pc-windows-gnu
# i686-pc-windows-msvc
# x86_64-pc-windows-gnu
# x86_64-pc-windows-msvc
# aarch64-pc-windows-msvc
```

---

## 5. Rustup — Toolchain Manager

### 5.1 What Rustup Does

**Rustup** is Rust's official toolchain installer and version manager (like `nvm` for Node or `pyenv` for Python).

**Key responsibilities:**
1. Install/update Rust toolchains (stable, beta, nightly)
2. Manage multiple toolchain versions
3. Install cross-compilation targets
4. Set default toolchains per project
5. Install components (rustfmt, clippy, rust-src)

### 5.2 Installing Rustup

**Linux/macOS:**
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

**Windows:**
```
Download: https://rustup.rs/
Run: rustup-init.exe
```

### 5.3 Essential Rustup Commands

**Install/update toolchains:**
```bash
rustup install stable         # Install stable channel
rustup install nightly        # Install nightly channel
rustup install 1.75.0         # Install specific version
rustup update                 # Update all installed toolchains
```

**Set default toolchain:**
```bash
rustup default stable         # Use stable by default
rustup default nightly        # Use nightly by default
```

**Per-directory override:**
```bash
cd my-project/
rustup override set nightly   # This project uses nightly
# Creates: rust-toolchain.toml
```

**List installed toolchains:**
```bash
rustup toolchain list
# Output:
# stable-x86_64-unknown-linux-gnu (default)
# nightly-x86_64-unknown-linux-gnu
```

**Show active toolchain:**
```bash
rustup show
# Output:
# Default host: x86_64-unknown-linux-gnu
# rustup home:  /home/user/.rustup
#
# installed toolchains
# --------------------
# stable-x86_64-unknown-linux-gnu (default)
# nightly-x86_64-unknown-linux-gnu
```

### 5.4 Components

**Install additional components:**
```bash
rustup component add rustfmt       # Code formatter
rustup component add clippy        # Linter
rustup component add rust-src      # Rust source code (for IDE)
rustup component add rust-analyzer # LSP server
```

**List installed components:**
```bash
rustup component list --installed
```

---

## 6. Installing Targets

### 6.1 What Is a Target?

A **target** is a pre-compiled standard library (`libstd`) for a specific platform. When you add a target, you're downloading:

```
┌────────────────────────────────────────────────────┐
│  libstd.rlib         ← Standard library            │
│  libcore.rlib        ← Core library (no_std)       │
│  liballoc.rlib       ← Allocator                   │
│  libpanic_unwind.rlib ← Panic handler              │
│  ...                                               │
└────────────────────────────────────────────────────┘
```

**You still need the system linker** — Rust doesn't ship linkers for every platform.

### 6.2 List Available Targets

```bash
rustup target list
# Shows all targets (200+), marking installed ones
```

**Filter for installed targets:**
```bash
rustup target list --installed
# Output:
# x86_64-unknown-linux-gnu (default)
```

### 6.3 Add a Target

```bash
rustup target add x86_64-pc-windows-gnu
```

**What this does:**
1. Downloads pre-compiled standard library for that target
2. Installs to: `~/.rustup/toolchains/<toolchain>/lib/rustlib/<target>/`
3. Now you can compile for Windows from Linux (if you have a linker)

**Example: Add multiple targets**
```bash
rustup target add x86_64-pc-windows-gnu
rustup target add aarch64-unknown-linux-gnu
rustup target add wasm32-unknown-unknown
```

### 6.4 Remove a Target

```bash
rustup target remove x86_64-pc-windows-gnu
```

### 6.5 Targets vs Toolchains

| Concept | What It Is | Example |
|---------|------------|---------|
| **Toolchain** | Rust compiler + cargo + std for **your OS** | `stable-x86_64-unknown-linux-gnu` |
| **Target** | Standard library for **another OS** | `x86_64-pc-windows-gnu` |

**Mental model:**
```
Toolchain (host):     Linux x86_64        ← rustc runs here
    ↓
 Compiles for
    ↓
Target:               Windows x86_64      ← binary runs here
```

---

## 7. Cargo Build with Targets

### 7.1 Basic Usage

**Build for another target:**
```bash
cargo build --target x86_64-pc-windows-gnu
```

**What happens:**
1. Rust compiles your code for Windows
2. Uses the Windows standard library (from `rustup target add`)
3. Calls the **MinGW linker** (`x86_64-w64-mingw32-ld`) to create `.exe`
4. Output: `target/x86_64-pc-windows-gnu/debug/program.exe`

### 7.2 Output Directory Structure

```
target/
├── debug/                          ← Native builds
│   └── program
└── x86_64-pc-windows-gnu/          ← Cross-compiled builds
    ├── debug/
    │   └── program.exe
    └── release/
        └── program.exe
```

### 7.3 Release Builds

```bash
cargo build --target x86_64-pc-windows-gnu --release
```

**Optimizations enabled:**
- `-C opt-level=3` (maximum optimization)
- `-C lto=fat` (link-time optimization, if enabled)
- Strip debug symbols
- Smaller, faster binary

### 7.4 Setting Default Target

**Option 1: `.cargo/config.toml` (project-specific)**
```toml
[build]
target = "x86_64-pc-windows-gnu"
```

Now `cargo build` automatically uses this target.

**Option 2: Environment variable**
```bash
export CARGO_BUILD_TARGET=x86_64-pc-windows-gnu
cargo build  # Uses Windows target
```

### 7.5 Linker Configuration

**Problem:** Cargo doesn't know where your cross-linker is.

**Solution:** Tell Cargo which linker to use in `.cargo/config.toml`:

```toml
[target.x86_64-pc-windows-gnu]
linker = "x86_64-w64-mingw32-gcc"

[target.aarch64-unknown-linux-gnu]
linker = "aarch64-linux-gnu-gcc"
```

**Why this works:**
- Rust generates `.o` files (LLVM backend)
- Then calls the specified linker to create the final executable
- The linker must understand the target platform's ABI

### 7.6 Complete Example — Linux → Windows

**Step 1: Install MinGW on Linux**
```bash
# Ubuntu/Debian:
sudo apt install gcc-mingw-w64-x86-64

# Arch:
sudo pacman -S mingw-w64-gcc

# Verify:
which x86_64-w64-mingw32-gcc
# Output: /usr/bin/x86_64-w64-mingw32-gcc
```

**Step 2: Add Rust target**
```bash
rustup target add x86_64-pc-windows-gnu
```

**Step 3: Configure linker**
Create `.cargo/config.toml`:
```toml
[target.x86_64-pc-windows-gnu]
linker = "x86_64-w64-mingw32-gcc"
```

**Step 4: Build**
```bash
cargo build --target x86_64-pc-windows-gnu --release
```

**Step 5: Test on Windows**
```bash
# Copy to Windows machine:
scp target/x86_64-pc-windows-gnu/release/program.exe user@windows-pc:

# Or use Wine on Linux:
wine target/x86_64-pc-windows-gnu/release/program.exe
```

---

## 8. Cross — Docker-Based Cross-Compiler

### 8.1 What Is Cross?

**Cross** is a tool that uses **Docker containers** to provide complete cross-compilation toolchains without manual setup.

**GitHub:** https://github.com/cross-rs/cross

**Key idea:**
```
┌──────────────────────────────────────────────────────┐
│  Your Linux machine                                  │
│    ↓                                                 │
│  cargo build --target aarch64-unknown-linux-gnu      │
│    ↓                                                 │
│  [ERROR: No linker for aarch64]                      │
└──────────────────────────────────────────────────────┘

WITH CROSS:
┌──────────────────────────────────────────────────────┐
│  Your Linux machine                                  │
│    ↓                                                 │
│  cross build --target aarch64-unknown-linux-gnu      │
│    ↓                                                 │
│  Docker container (has aarch64-linux-gnu-gcc)        │
│    ↓                                                 │
│  ✅ Binary compiled successfully                     │
└──────────────────────────────────────────────────────┘
```

### 8.2 Installing Cross

```bash
cargo install cross --git https://github.com/cross-rs/cross
```

**Requirements:**
- Docker or Podman installed and running
- User in `docker` group (Linux)

**Verify:**
```bash
cross --version
# Output: cross 0.2.x
```

### 8.3 How Cross Works

**Architecture:**
```
┌─────────────────────────────────────────────────────────┐
│  1. You run: cross build --target <TARGET>             │
│     ↓                                                   │
│  2. Cross checks if <TARGET> needs Docker               │
│     ↓                                                   │
│  3. Cross pulls pre-built Docker image:                 │
│     ghcr.io/cross-rs/aarch64-unknown-linux-gnu:latest  │
│     ↓                                                   │
│  4. Image contains:                                     │
│     • GCC cross-compiler                                │
│     • System libraries                                  │
│     • Rust toolchain + std for target                   │
│     ↓                                                   │
│  5. Cross mounts your project into container            │
│     ↓                                                   │
│  6. Runs cargo build inside container                   │
│     ↓                                                   │
│  7. Binary written to target/ (visible on host)         │
└─────────────────────────────────────────────────────────┘
```

### 8.4 Basic Usage

**Instead of:**
```bash
cargo build --target aarch64-unknown-linux-gnu
```

**Use:**
```bash
cross build --target aarch64-unknown-linux-gnu
```

**All cargo commands work:**
```bash
cross build --release
cross test --target armv7-unknown-linux-gnueabihf
cross run --target x86_64-pc-windows-gnu
```

### 8.5 Supported Targets

Cross has pre-built images for **most major targets**:

| Target | Docker Image |
|--------|--------------|
| `aarch64-unknown-linux-gnu` | ARM 64-bit Linux |
| `armv7-unknown-linux-gnueabihf` | ARM 32-bit Linux (hard float) |
| `x86_64-pc-windows-gnu` | Windows 64-bit (MinGW) |
| `mips-unknown-linux-gnu` | MIPS Linux |
| `riscv64gc-unknown-linux-gnu` | RISC-V 64-bit |

**Full list:**
```bash
cross build --target help
```

### 8.6 Custom Docker Images — Cross.toml

**Problem:** Your project needs custom libraries (e.g., OpenSSL for ARM).

**Solution:** Create a `Cross.toml` file:

```toml
[target.aarch64-unknown-linux-gnu]
image = "my-custom-cross-image:latest"

[target.aarch64-unknown-linux-gnu.env]
passthrough = [
    "MY_ENV_VAR",
]
```

**Build custom image:**
```dockerfile
# Dockerfile.cross
FROM ghcr.io/cross-rs/aarch64-unknown-linux-gnu:latest

RUN dpkg --add-architecture arm64 && \
    apt-get update && \
    apt-get install -y libssl-dev:arm64
```

```bash
docker build -f Dockerfile.cross -t my-custom-cross-image:latest .
```

### 8.7 Cross vs Manual Setup

| Aspect | Manual Setup | Cross |
|--------|--------------|-------|
| **Setup time** | Hours (install toolchains) | Minutes (pull Docker image) |
| **Reproducibility** | Hard (different systems) | Easy (same Docker image) |
| **Disk space** | Minimal (just toolchain) | Large (Docker images ~1-2GB each) |
| **Performance** | Native | Slight overhead (Docker I/O) |
| **Complexity** | High (linker paths, etc.) | Low (just works) |

### 8.8 When to Use Cross

**Use Cross when:**
- ✅ Cross-compiling to many targets (ARM, MIPS, Windows, etc.)
- ✅ You don't want to install toolchains manually
- ✅ CI/CD pipelines (consistent environment)
- ✅ Complex C dependencies

**Don't use Cross when:**
- ❌ Single target, already have toolchain installed
- ❌ Minimal disk space (Docker images are large)
- ❌ Building for the same architecture (use native cargo)

---

## 9. Complete Examples

### 9.1 Linux → Windows (MinGW)

**Goal:** Compile Rust binary on Linux, run on Windows.

**Step 1: Install MinGW**
```bash
sudo apt install gcc-mingw-w64-x86-64
```

**Step 2: Add Rust target**
```bash
rustup target add x86_64-pc-windows-gnu
```

**Step 3: Configure linker**
`.cargo/config.toml`:
```toml
[target.x86_64-pc-windows-gnu]
linker = "x86_64-w64-mingw32-gcc"
```

**Step 4: Build**
```bash
cargo build --target x86_64-pc-windows-gnu --release
```

**Output:** `target/x86_64-pc-windows-gnu/release/program.exe`

### 9.2 macOS → Linux (Cross)

**Goal:** Compile on macOS, run on Linux.

**Step 1: Install Cross**
```bash
cargo install cross --git https://github.com/cross-rs/cross
```

**Step 2: Build**
```bash
cross build --target x86_64-unknown-linux-gnu --release
```

**Output:** `target/x86_64-unknown-linux-gnu/release/program`

**Copy to Linux:**
```bash
scp target/x86_64-unknown-linux-gnu/release/program user@linux-server:
```

### 9.3 Linux → ARM Raspberry Pi

**Option A: Using Cross**
```bash
cross build --target aarch64-unknown-linux-gnu --release
# Copy to Pi:
scp target/aarch64-unknown-linux-gnu/release/program pi@192.168.1.x:
```

**Option B: Manual setup**
```bash
# Install ARM toolchain:
sudo apt install gcc-aarch64-linux-gnu

# Add target:
rustup target add aarch64-unknown-linux-gnu

# Configure linker:
# .cargo/config.toml
[target.aarch64-unknown-linux-gnu]
linker = "aarch64-linux-gnu-gcc"

# Build:
cargo build --target aarch64-unknown-linux-gnu --release
```

### 9.4 Static Binary (musl)

**Goal:** Single binary with no dynamic dependencies.

```bash
# Add musl target:
rustup target add x86_64-unknown-linux-musl

# Install musl tools:
sudo apt install musl-tools

# Build:
cargo build --target x86_64-unknown-linux-musl --release

# Result: Fully static binary
ldd target/x86_64-unknown-linux-musl/release/program
# Output: not a dynamic executable
```

---

## 10. Troubleshooting

### 10.1 Error: Linker Not Found

**Error:**
```
error: linker `x86_64-w64-mingw32-gcc` not found
```

**Cause:** Cross-compiler not installed.

**Fix:**
```bash
# Linux:
sudo apt install gcc-mingw-w64-x86-64

# Verify:
which x86_64-w64-mingw32-gcc
```

### 10.2 Error: Linking with `cc` Failed

**Error:**
```
= note: /usr/bin/ld: cannot find -lpthread
```

**Cause:** Missing system libraries for target.

**Fix with Cross:**
```bash
cross build --target x86_64-unknown-linux-gnu
# Cross provides all libraries in Docker
```

### 10.3 Error: Can't Execute Windows Binary

**Error:**
```
bash: ./program.exe: cannot execute binary file: Exec format error
```

**Cause:** Trying to run Windows binary on Linux.

**Fix:**
```bash
# Option 1: Use Wine:
wine ./program.exe

# Option 2: Copy to Windows machine:
scp ./program.exe user@windows:
```

### 10.4 Cross: Permission Denied (Docker)

**Error:**
```
docker: Got permission denied while trying to connect to the Docker daemon socket
```

**Fix:**
```bash
# Add user to docker group:
sudo usermod -aG docker $USER

# Log out and back in, or:
newgrp docker

# Verify:
docker ps
```

### 10.5 OpenSSL Linking Errors

**Error:**
```
error: failed to run custom build command for `openssl-sys`
```

**Fix for Cross:**
```toml
# Cargo.toml
[dependencies]
openssl = { version = "0.10", features = ["vendored"] }
```

**Or use rustls instead:**
```toml
[dependencies]
reqwest = { version = "0.11", default-features = false, features = ["rustls-tls"] }
```

### 10.6 Slow Cross Builds

**Problem:** First build takes 10+ minutes.

**Cause:** Docker pulling 2GB image.

**Fix:**
```bash
# Pre-pull images:
docker pull ghcr.io/cross-rs/aarch64-unknown-linux-gnu:latest
docker pull ghcr.io/cross-rs/x86_64-pc-windows-gnu:latest
```

### 10.7 Cargo.toml Doesn't Recognize [build]

**Error:**
```toml
[build]
target = "x86_64-pc-windows-gnu"  # ❌ ERROR: unknown field `build`
```

**Fix:** Use `.cargo/config.toml` instead:
```toml
[build]
target = "x86_64-pc-windows-gnu"
```

---

## Quick Reference

### Essential Commands

```bash
# Toolchain management
rustup install stable
rustup default stable
rustup toolchain list

# Target management
rustup target list
rustup target add x86_64-pc-windows-gnu
rustup target remove x86_64-pc-windows-gnu

# Building
cargo build --target x86_64-pc-windows-gnu
cargo build --target x86_64-pc-windows-gnu --release

# Cross (Docker-based)
cargo install cross --git https://github.com/cross-rs/cross
cross build --target aarch64-unknown-linux-gnu
```

### Target Cheat Sheet

```
Linux    → Windows:  x86_64-pc-windows-gnu
Linux    → macOS:    x86_64-apple-darwin (requires macOS SDK)
macOS    → Linux:    x86_64-unknown-linux-gnu
Windows  → Linux:    x86_64-unknown-linux-gnu (use Cross in WSL)
Any      → ARM:      aarch64-unknown-linux-gnu
Any      → WASM:     wasm32-unknown-unknown
```

### Linker Configuration Template

`.cargo/config.toml`:
```toml
[build]
# Optional: set default target
# target = "x86_64-pc-windows-gnu"

[target.x86_64-pc-windows-gnu]
linker = "x86_64-w64-mingw32-gcc"

[target.aarch64-unknown-linux-gnu]
linker = "aarch64-linux-gnu-gcc"

[target.armv7-unknown-linux-gnueabihf]
linker = "arm-linux-gnueabihf-gcc"
```

---

**Sources:** GCC documentation, Clang/LLVM project, MinGW-w64 wiki, Rust reference, Cross GitHub, rustup book.
