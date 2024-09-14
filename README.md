# Util Macros

### Create blocks for your Rust code to group your functions, structs and others.

---

## Usage

### To use `util_macros`, follow these steps

### 1. Add `util_macros` to your Cargo.toml
```toml
[package]
name = "my_rust_program"
version = "0.1.0"
edition = "2021"

[dependencies]
util_macros = { git = "https://github.com/ItsMeViipeR/util_macros", branch = "master" }
```

### 2. Import `util_macros`
```rust
use util_macros::*; // you can just import a macro if you want
```

### 3. Use the macro(s) you just imported

```rust
use util_macros::functions;

functions! {
    greet(name: String) {
        println!("Hello {}", name);
    }
}

fn main() {
    greet("Bob".to_owned())
}
```

## Mutability

### If you want to add `mut` keyword to your function, you have to do it this way

```rust
use util_macros::*;

structs! {
    User { name: String }
}

functions! {
    create_user(name: String) -> User { User { name } }
    update_user(user: &mut User, name: String) -> () { user.name = name }
}

fn main() {
    let mut user: User = create_user("ViipeR".to_owned());

    println!("Created user: {}", user.name);

    update_user(&mut user, "ViipeR".to_owned());
}
```