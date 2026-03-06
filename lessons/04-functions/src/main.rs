fn main() {
    println!("=== Functions Demo ===");
    
    // Function with no parameters or return value
    greet();
    
    // Function with parameters
    greet_person("Alice");
    
    // Function with return value
    let sum = add(5, 3);
    println!("5 + 3 = {}", sum);
    
    // Function with multiple parameters and return
    let result = calculate(10, 5);
    println!("Calculate result: {}", result);
    
    // Expression vs statement
    let y = {
        let x = 3;
        x + 1  // Expression without semicolon
    };
    println!("Value from block expression: {}", y);
}

fn greet() {
    println!("Hello from a function!");
}

fn greet_person(name: &str) {
    println!("Hello, {}!", name);
}

fn add(a: i32, b: i32) -> i32 {
    a + b  // Return value (no semicolon)
}

fn calculate(x: i32, y: i32) -> i32 {
    let result = x * 2 + y;
    result  // Implicit return
}
