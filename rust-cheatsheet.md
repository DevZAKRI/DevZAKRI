# 🦀 Rust Cheatsheet for JavaScript Developers

## Quick Comparison: JavaScript vs Rust

| Concept | JavaScript | Rust |
|---------|------------|------|
| **Variables** | `let x = 5; x = 10;` | `let mut x = 5; x = 10;` |
| **Constants** | `const X = 5;` | `const X: i32 = 5;` |
| **Functions** | `function add(a, b) { return a + b; }` | `fn add(a: i32, b: i32) -> i32 { a + b }` |
| **Arrays** | `let arr = [1, 2, 3];` | `let arr: [i32; 3] = [1, 2, 3];` |
| **Dynamic Arrays** | `let arr = [1, 2, 3]; arr.push(4);` | `let mut vec = vec![1, 2, 3]; vec.push(4);` |
| **Objects** | `let obj = { name: "John", age: 30 };` | `struct Person { name: String, age: u32 }` |
| **Error Handling** | `try { ... } catch(e) { ... }` | `match result { Ok(val) => ..., Err(e) => ... }` |

## 🔧 Essential Rust Concepts

### 1. Ownership & Borrowing (Unique to Rust!)
```rust
// Move (transfers ownership)
let s1 = String::from("hello");
let s2 = s1; // s1 is no longer valid

// Clone (creates a copy)
let s3 = String::from("hello");
let s4 = s3.clone(); // Both s3 and s4 are valid

// Borrow (temporary access)
let s5 = String::from("hello");
let len = calculate_length(&s5); // s5 is still valid
```

### 2. Data Types
```rust
// Numbers
let integer: i32 = 42;
let unsigned: u32 = 42;
let float: f64 = 3.14;

// Strings
let string_literal: &str = "Hello";
let owned_string: String = String::from("Hello");

// Arrays vs Vectors
let array: [i32; 3] = [1, 2, 3];        // Fixed size
let vector: Vec<i32> = vec![1, 2, 3];   // Dynamic size

// Optional values (like null/undefined in JS)
let some_value: Option<i32> = Some(5);
let no_value: Option<i32> = None;
```

### 3. Error Handling
```rust
// Result type (instead of try/catch)
fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("Division by zero".to_string())
    } else {
        Ok(a / b)
    }
}

// Handling results
match divide(10.0, 2.0) {
    Ok(result) => println!("Result: {}", result),
    Err(error) => println!("Error: {}", error),
}

// Or use ? operator for early return
fn calculate() -> Result<f64, String> {
    let result = divide(10.0, 2.0)?; // Returns early if error
    Ok(result * 2.0)
}
```

### 4. Pattern Matching
```rust
let number = 5;

match number {
    1 => println!("One"),
    2..=5 => println!("Between 2 and 5"),
    _ => println!("Something else"),
}

// Destructuring (like JS)
let tuple = (1, "hello", true);
let (x, y, z) = tuple;
```

### 5. Iterators (Similar to JS Array Methods)
```rust
let numbers = vec![1, 2, 3, 4, 5];

// Map (like JS map)
let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();

// Filter (like JS filter)
let evens: Vec<&i32> = numbers.iter().filter(|&x| x % 2 == 0).collect();

// Reduce (like JS reduce)
let sum: i32 = numbers.iter().sum();

// For each (like JS forEach)
numbers.iter().for_each(|x| println!("{}", x));
```

### 6. Structs & Implementations
```rust
// Define a struct (like class in JS)
struct User {
    name: String,
    email: String,
    age: u32,
}

// Implementation (like class methods)
impl User {
    // Constructor (like JS class constructor)
    fn new(name: String, email: String, age: u32) -> User {
        User { name, email, age }
    }
    
    // Method (like JS class method)
    fn greet(&self) -> String {
        format!("Hello, my name is {}", self.name)
    }
}

// Usage
let user = User::new("DevZAKRI".to_string(), "email@example.com".to_string(), 25);
println!("{}", user.greet());
```

### 7. Enums (More Powerful than JS)
```rust
enum Status {
    Active,
    Inactive,
    Pending(String), // Can hold data!
}

let status = Status::Pending("Waiting for approval".to_string());

match status {
    Status::Active => println!("User is active"),
    Status::Inactive => println!("User is inactive"),
    Status::Pending(msg) => println!("Pending: {}", msg),
}
```

## 🚀 Common Patterns

### 1. Working with JSON (like in Node.js)
```rust
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct ApiResponse {
    status: String,
    data: Vec<String>,
}

// Serialize to JSON
let response = ApiResponse {
    status: "success".to_string(),
    data: vec!["item1".to_string(), "item2".to_string()],
};
let json = serde_json::to_string(&response).unwrap();

// Deserialize from JSON
let parsed: ApiResponse = serde_json::from_str(&json).unwrap();
```

### 2. Async/Await (like in JS)
```rust
async fn fetch_data() -> Result<String, reqwest::Error> {
    let response = reqwest::get("https://api.example.com/data").await?;
    let text = response.text().await?;
    Ok(text)
}

#[tokio::main]
async fn main() {
    match fetch_data().await {
        Ok(data) => println!("Data: {}", data),
        Err(e) => println!("Error: {}", e),
    }
}
```

### 3. File I/O
```rust
use std::fs;

// Read file
let content = fs::read_to_string("file.txt")
    .expect("Failed to read file");

// Write file
fs::write("output.txt", "Hello, Rust!")
    .expect("Failed to write file");
```

## 📚 Essential Commands

```bash
# Create new project
cargo new my-project --bin

# Build project
cargo build

# Run project
cargo run

# Run tests
cargo test

# Add dependency
cargo add serde

# Update dependencies
cargo update

# Check code without building
cargo check

# Format code
cargo fmt

# Lint code
cargo clippy
```

## 🔗 Useful Crates (like npm packages)

| Purpose | Crate | Description |
|---------|-------|-------------|
| **Web Framework** | `axum` | Fast web framework (like Express) |
| **Async Runtime** | `tokio` | Async runtime (like Node.js event loop) |
| **HTTP Client** | `reqwest` | HTTP client (like axios) |
| **JSON** | `serde_json` | JSON serialization (like JSON.parse/stringify) |
| **Database** | `sqlx` | SQL database toolkit |
| **CLI** | `clap` | Command line argument parser |
| **Date/Time** | `chrono` | Date and time (like moment.js) |
| **Testing** | `mockall` | Mocking framework |
| **Logging** | `tracing` | Structured logging |

## 🎯 Learning Path for JS Developers

1. **Start Here**: Basic syntax and ownership concepts
2. **Build**: Simple CLI tools and algorithms
3. **Web Development**: REST APIs with Axum
4. **Advanced**: Concurrency, macros, and unsafe code
5. **Real Projects**: Build web services, CLI tools, or systems software

## 💡 Key Differences to Remember

- **Memory Management**: Rust manages memory automatically without GC
- **Type Safety**: Compile-time guarantees prevent many runtime errors
- **Performance**: Generally faster than JavaScript
- **Learning Curve**: Steeper initially, but very rewarding
- **Ecosystem**: Smaller but high-quality crate ecosystem

## 🔧 VS Code Extensions for Rust

1. **rust-analyzer** - Language server
2. **CodeLLDB** - Debugger
3. **crates** - Manage dependencies
4. **Better TOML** - TOML syntax highlighting
5. **Error Lens** - Inline error display

Remember: Rust's compiler is your friend! It catches errors early and provides helpful suggestions. Don't fight the borrow checker - learn from it! 🦀