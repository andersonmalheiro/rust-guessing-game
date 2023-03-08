pub fn execute() {
    // Immutable variable
    let x = 5;
    println!("The value of x is: {x}");

    // Mutable variable
    let mut y = 10;
    println!("The value of y is: {y}");

    y = 12;
    println!("The new value of y is: {y}");

    // const declaration
    // Must follow this naming convention, uppercase with underscore separating words
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("3 hours in seconds -> {THREE_HOURS_IN_SECONDS}");

    // Shadowing
    // Shadowing allow us to redeclare the same variable with a different value
    let spaces = "   ";
    let spaces = spaces.len();
    println!("Spaces -> {spaces}")
}
