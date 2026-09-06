fn scope() {
    // This binding lives in main function
    let long_lived_binding = 1;

    // This block has a smaller scope than the main function
    {
        let short_lived_binding = 2;
        println!("inner short: {}", short_lived_binding);
    }

    // short_lived_binding doesn't exist in this outer scope

    println!("outer long: {}", long_lived_binding);
}

pub fn execute() {
    scope();
}

// Output:
// inner short: 2
// outer long: 1
