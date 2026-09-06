fn declare_first() {
    // It's possible to declare the variable binding first and
    // Initialize them later
    // Declare a variable binding
    let a_binding;

    {
        let x = 2;
        // Initialize the binding
        // Accessing the binding before declaration will raise an error
        a_binding = x * x;
    }

    println!("a binding: {}", a_binding);
}

pub fn execute() {
    declare_first();
}

// Output:
// a binding: 4
