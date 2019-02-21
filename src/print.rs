pub fn run() {
    // print to console
    println!("Printing from the print.rs file.");

    // formatting
    println!("Number: {}", 1);
    println!("{} is learning {}.", "Florian", "Rust");

    // positional arguments
    println!(
        "{0} is writing {1} code and {0} enjoys {1}",
        "Florian", "Rust"
    );

    // named arguments
    println!(
        "{name} is writing {lang1} code and {name} also knows {lang2}, {lang3} and {lang4}.",
        name = "Florian",
        lang1 = "Rust",
        lang2 = "JavaScript",
        lang3 = "Go",
        lang4 = "Python"
    );

    // Placeholder traits
    println!(
        "The decimal 10 is equivalent to Binary: {:b}, Hex: {:x} and Octal: {:o}",
        10, 10, 10
    );

    // placeholder for debug traits
    println!("{:?}", (99, false, "Hujambo"));

    // Math
    println!("99 + 44 = {}", 99 + 44);
    println!("99 - 44 = {}", 99 - 44);
    println!("99 * 44 = {}", 99 * 44);
    println!("99 / 44 = {}", 99 / 44);
    println!("99 % 44 = {}", 99 % 44);
}
