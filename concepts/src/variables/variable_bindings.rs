fn var_binding() {
    let an_integer = 1u32;
    let a_boolean = true;
    let unit = ();

    // Copy an_integer into copied_integer
    let copied_integer = an_integer;

    println!("An integer: {}", copied_integer);
    println!("A boolean: {}", a_boolean);
    println!("Meet the unit value: {:?}", unit);

    // The compiler warns about unused variable bindings; these warnings can
    // be silenced by prefixing the variable name with an underscore
    let _unused_variable = 3u32;

    // Mutability
    // Variable bindings are immutable by default, but this can be
    // overridden using the mut modifier
    let mut mutable_binding = 1;

    println!("Before mutation: {}", mutable_binding);
    
    // Ok
    mutable_binding += 1;

    println!("After mutation: {}", mutable_binding);
}

pub fn execute() {
    var_binding();
}

// Output:
// An integer: 1
// A boolean: true
// Meet the unit value: ()
// Before mutation: 1
// After mutation: 2
