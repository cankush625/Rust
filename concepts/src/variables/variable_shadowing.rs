fn shadow() {
    let shadowed_binding = 1;

    {
        println!("Before being shadowed: {}", shadowed_binding);

        // This binding shadows the outer one
        let shadowed_binding = "abc";

        println!("Shadowed in inner block: {}", shadowed_binding);
    }

    println!("Outside inner block: {}", shadowed_binding);
    // This binding *shadows* the previous binding
    let shadowed_binding = 2;
    println!("Shadowed in outer block: {}", shadowed_binding);
}

pub fn execute() {
    shadow();
}

// Output:
// Before being shadowed: 1
// Shadowed in inner block: abc
// Outside inner block: 1
// Shadowed in outer block: 2
