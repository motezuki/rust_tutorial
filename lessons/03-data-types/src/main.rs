fn main() {
    // Integer types
    let integer: i32 = 42;
    println!("Integer: {}", integer);
    
    // Floating point
    let float: f64 = 3.14;
    println!("Float: {}", float);
    
    // Boolean
    let is_active: bool = true;
    println!("Boolean: {}", is_active);
    
    // Character
    let letter: char = 'A';
    let emoji: char = '😀';
    println!("Character: {}, Emoji: {}", letter, emoji);
    
    // Tuple
    let tuple: (i32, f64, char) = (500, 6.4, 'x');
    println!("Tuple: ({}, {}, {})", tuple.0, tuple.1, tuple.2);
    
    // Array
    let array: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Array: {:?}", array);
    println!("First element: {}", array[0]);
    
    // String slice
    let text: &str = "Hello, Rust!";
    println!("String slice: {}", text);
}
