// Rust Learning Examples for JavaScript Developers
// Run with: cargo run

fn main() {
    println!("🦀 Welcome to Rust! Coming from JavaScript? Let's explore the differences!\n");
    
    // 1. Variables and Mutability (different from JS let/const)
    variables_and_mutability();
    
    // 2. Data Types (more explicit than JS)
    data_types();
    
    // 3. Functions (similar to JS but with types)
    function_examples();
    
    // 4. Ownership (unique to Rust)
    ownership_basics();
    
    // 5. Error Handling (different from try/catch)
    error_handling();
    
    // 6. Collections (similar to JS arrays/objects)
    collections_demo();
    
    // 7. Async Programming (similar to JS async/await)
    // async_demo(); // We'll add this later
}

fn variables_and_mutability() {
    println!("📝 Variables and Mutability:");
    
    // In JS: let x = 5; x = 10; // works
    // In Rust: variables are immutable by default
    let x = 5;
    println!("  Immutable x: {}", x);
    
    // To make it mutable, use 'mut'
    let mut y = 10;
    println!("  Mutable y before: {}", y);
    y = 20;
    println!("  Mutable y after: {}", y);
    
    // Shadowing (different from JS)
    let z = 5;
    let z = z + 1; // This creates a new variable
    println!("  Shadowed z: {}", z);
    println!();
}

fn data_types() {
    println!("🔢 Data Types (more explicit than JS):");
    
    // Numbers (JS has just 'number', Rust has many)
    let integer: i32 = 42;
    let float: f64 = 3.14;
    let boolean: bool = true;
    
    // Strings (more complex than JS strings)
    let string_slice: &str = "Hello"; // String literal
    let owned_string: String = String::from("World"); // Owned string
    
    println!("  Integer: {}", integer);
    println!("  Float: {}", float);
    println!("  Boolean: {}", boolean);
    println!("  String slice: {}", string_slice);
    println!("  Owned string: {}", owned_string);
    
    // Arrays (fixed size, unlike JS arrays)
    let array: [i32; 3] = [1, 2, 3];
    println!("  Array: {:?}", array);
    
    // Tuples (similar to JS arrays but typed)
    let tuple: (i32, f64, bool) = (42, 3.14, true);
    println!("  Tuple: {:?}", tuple);
    println!();
}

fn function_examples() {
    println!("🔧 Functions:");
    
    // Unlike JS, Rust functions must declare parameter and return types
    fn add(a: i32, b: i32) -> i32 {
        a + b // No 'return' needed for last expression
    }
    
    // Function with explicit return
    fn multiply(a: i32, b: i32) -> i32 {
        return a * b;
    }
    
    let sum = add(5, 3);
    let product = multiply(4, 6);
    
    println!("  5 + 3 = {}", sum);
    println!("  4 * 6 = {}", product);
    
    // Closures (similar to JS arrow functions)
    let closure = |x: i32| x * 2;
    println!("  Closure result: {}", closure(5));
    println!();
}

fn ownership_basics() {
    println!("🏠 Ownership (Rust's unique feature):");
    
    // This concept doesn't exist in JS!
    let s1 = String::from("Hello");
    let s2 = s1; // s1 is moved to s2, s1 is no longer valid
    
    // println!("{}", s1); // This would cause a compile error!
    println!("  Moved string: {}", s2);
    
    // To avoid moving, use clone
    let s3 = String::from("World");
    let s4 = s3.clone(); // Both s3 and s4 are valid
    
    println!("  Original: {}", s3);
    println!("  Cloned: {}", s4);
    
    // References (borrowing)
    let s5 = String::from("Borrowing");
    let len = calculate_length(&s5); // Borrowing, not moving
    println!("  String '{}' has length {}", s5, len);
    println!();
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn error_handling() {
    println!("⚠️  Error Handling (different from try/catch):");
    
    // Rust uses Result<T, E> instead of exceptions
    fn divide(a: f64, b: f64) -> Result<f64, String> {
        if b == 0.0 {
            Err("Division by zero!".to_string())
        } else {
            Ok(a / b)
        }
    }
    
    // Handle the result
    match divide(10.0, 2.0) {
        Ok(result) => println!("  10 / 2 = {}", result),
        Err(error) => println!("  Error: {}", error),
    }
    
    match divide(10.0, 0.0) {
        Ok(result) => println!("  10 / 0 = {}", result),
        Err(error) => println!("  Error: {}", error),
    }
    
    // Using unwrap (like JS throw if error)
    let good_result = divide(8.0, 4.0).unwrap();
    println!("  8 / 4 = {}", good_result);
    println!();
}

fn collections_demo() {
    println!("📦 Collections:");
    
    // Vec (similar to JS arrays)
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);
    println!("  Vector: {:?}", vec);
    
    // HashMap (similar to JS objects/Maps)
    use std::collections::HashMap;
    let mut map = HashMap::new();
    map.insert("name", "DevZAKRI");
    map.insert("language", "Rust");
    
    println!("  HashMap:");
    for (key, value) in &map {
        println!("    {}: {}", key, value);
    }
    
    // Iterators (similar to JS array methods)
    let numbers = vec![1, 2, 3, 4, 5];
    let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();
    println!("  Original: {:?}", numbers);
    println!("  Doubled: {:?}", doubled);
    
    // Filter (like JS filter)
    let evens: Vec<&i32> = numbers.iter().filter(|&x| x % 2 == 0).collect();
    println!("  Even numbers: {:?}", evens);
    println!();
}
