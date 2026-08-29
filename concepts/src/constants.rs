static LANGUAGE: &str = "Rust";
const THRESHOLD: i32 = 5;

fn is_big(n: i32) -> bool {
    n > THRESHOLD
}

pub fn execute() {
    let n = 16;

    // Access constant in the main thread
    println!("This is {}", LANGUAGE);
    println!("The threshold is {}", THRESHOLD);
    println!("{} is {}", n, if is_big(n) { "big" } else { "small" });

    // Error! Cannot modify a `const`.
    // THRESHOLD = 5;
    // FIXME ^ Comment out this line
}

// Output:
// This is Rust
// The threshold is 5
// 16 is big
