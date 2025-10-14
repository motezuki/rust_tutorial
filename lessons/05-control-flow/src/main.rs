fn main() {
    println!("=== Control Flow Demo ===\n");
    
    // If expressions
    let number = 7;
    if number < 5 {
        println!("{} is less than 5", number);
    } else if number > 5 {
        println!("{} is greater than 5", number);
    } else {
        println!("{} is equal to 5", number);
    }
    
    // If in a let statement
    let condition = true;
    let value = if condition { 5 } else { 6 };
    println!("Value from if expression: {}\n", value);
    
    // Loop
    println!("Loop example:");
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 3 {
            break counter * 2;
        }
    };
    println!("Loop result: {}\n", result);
    
    // While loop
    println!("While loop example:");
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("Liftoff!\n");
    
    // For loop
    println!("For loop example:");
    let array = [10, 20, 30, 40, 50];
    for element in array {
        println!("Value: {}", element);
    }
    
    // For loop with range
    println!("\nFor loop with range:");
    for number in 1..4 {
        println!("Number: {}", number);
    }
}
