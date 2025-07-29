# 🦀 Getting Started with Rust - DevZAKRI's Setup

Welcome to your Rust development environment! Since you're coming from JavaScript, this guide will help you get started quickly.

## 🚀 What's Been Set Up

### 1. Rust Toolchain
- **Rust 1.82.0** - Latest stable version
- **Cargo** - Package manager and build tool (like npm)
- **Rustup** - Toolchain manager

### 2. Learning Projects Created

#### 📚 `rust-learning/` - Basic Concepts
A comprehensive learning project that demonstrates Rust concepts with JavaScript comparisons.

**To run:**
```bash
cd rust-learning
cargo run
```

**What you'll learn:**
- Variables and mutability
- Data types vs JavaScript
- Functions and closures
- Ownership and borrowing (unique to Rust!)
- Error handling without try/catch
- Collections and iterators

#### 🌐 `web-server-rust/` - Web Development
A REST API server similar to Express.js but with Rust's performance and safety.

**To run:**
```bash
cd web-server-rust
cargo run
```

**Features:**
- RESTful API endpoints
- JSON serialization/deserialization
- CORS support
- State management
- Error handling

**Test the API:**
```bash
# Get all users
curl http://127.0.0.1:3000/api/users

# Get specific user
curl http://127.0.0.1:3000/api/users/1

# Create new user
curl -X POST http://127.0.0.1:3000/api/users \
  -H "Content-Type: application/json" \
  -d '{"name":"New User","email":"new@example.com","age":25}'
```

#### 🔧 `cli-tool/` - Command Line Application
A practical CLI tool for JSON file operations (like Node.js CLI tools).

**To run:**
```bash
cd cli-tool
cargo run -- --help
```

**Examples:**
```bash
# Create a sample JSON file
cargo run -- create -f sample.json

# Read and display the file
cargo run -- read -f sample.json

# List all JSON files
cargo run -- list

# Pretty print JSON
cargo run -- pretty -f sample.json

# Analyze file statistics
cargo run -- analyze -f sample.json
```

## 📖 Learning Resources

### Essential Commands
```bash
# Create new project
cargo new my-project --bin

# Build project
cargo build

# Run project
cargo run

# Run tests
cargo test

# Add dependencies
cargo add serde tokio axum

# Format code
cargo fmt

# Check for issues
cargo clippy
```

### Recommended Learning Path

1. **Start with `rust-learning`** - Run it and read the code
2. **Study `rust-cheatsheet.md`** - JavaScript to Rust comparisons
3. **Experiment with `cli-tool`** - Practical file operations
4. **Build the web server** - Familiar Express.js-like concepts
5. **Create your own projects** - Apply what you've learned

## 🔗 Key Differences from JavaScript

| JavaScript | Rust | Why? |
|------------|------|------|
| `let x = 5; x = 10;` | `let mut x = 5; x = 10;` | Explicit mutability |
| `try/catch` | `Result<T, E>` | No exceptions, explicit error handling |
| Garbage collected | Ownership system | Zero-cost memory management |
| Dynamic typing | Static typing | Compile-time safety |
| Runtime errors | Compile-time errors | Catch bugs early |

## 💡 Tips for JavaScript Developers

1. **Embrace the compiler** - It's your friend, not an enemy
2. **Think about ownership** - Who owns this data?
3. **Use `Result` and `Option`** - Instead of null/undefined
4. **Start small** - Build simple programs first
5. **Read error messages** - Rust gives helpful suggestions

## 🎯 Next Steps

1. Run all three projects to see them in action
2. Modify the code to experiment
3. Read the cheatsheet for quick reference
4. Build a simple project of your own
5. Join the Rust community on Discord/Reddit

## 📚 Additional Resources

- [The Rust Book](https://doc.rust-lang.org/book/) - Official guide
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/) - Learn by coding
- [Rustlings](https://github.com/rust-lang/rustlings) - Interactive exercises
- [Rust Discord](https://discord.gg/rust-lang) - Community help

Remember: Rust has a steeper learning curve than JavaScript, but the benefits are huge - memory safety, performance, and fearless concurrency! 🚀

Happy coding! 🦀