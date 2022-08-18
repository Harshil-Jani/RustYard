pub fn run() {
    // Print to console
    println!("Hello from print.rs file !");

    // Basic Formatting
    println!("{} is {}.", "Harshil", 19);

    // Positional Arguments
    println!(
        "{0} is from {1} and {0} likes to {2}",
        "Harshil", "SVNIT", "OpenSource"
    );

    // Named Arguments
    println!(
        "{name} likes to {activity}",
        name = "Harshil",
        activity = "OpenSource"
    );

    // Placeholder traits
    println!("Binary : {:b}, Hex: {:x}, Octal {:o}", 10, 10, 10);

    // Placeholder for debug traits : Actually known as Tuple
    println!("{:?}", (12, true, "Harshil"));

    // Basic Math
    println!("10+10 = {}",10+10);
}
