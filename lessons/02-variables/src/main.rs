fn main() {
    // Immutable variable
    let x = 5;
    println!("The value of x is: {}", x);
    
    // Mutable variable
    let mut y = 10;
    println!("The value of y is: {}", y);
    y = 15;
    println!("The new value of y is: {}", y);
    
    // Shadowing
    let z = 20;
    println!("The value of z is: {}", z);
    let z = z + 5;
    println!("The shadowed value of z is: {}", z);
    
    // Constants
    const MAX_POINTS: u32 = 100_000;
    println!("The maximum points are: {}", MAX_POINTS);
}
