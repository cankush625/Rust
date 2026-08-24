// An attribute to hide warnings for unused code.
#![allow(dead_code)]

// There are three types of Structures that can be created
// using word struct
// 1. Tuple Structs -> These are basically named tuples
// 2. C Structs -> The classic C style Structs
// 3. Unit Structs -> These are field-less, are useful for generics
#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

// A unit struct
struct Unit;

// A tuple struct
struct Pair(i32, f32);

// A struct with two fields
struct Point {
    x: f32,
    y: f32,
}

// Structs can be used as fields of another struct
struct Rectangle {
    // A rectangle can be specified by where the top left and bottom right
    // corners are in space.
    top_left: Point,
    bottom_right: Point,
}

pub fn execute() {
    // Create struct with field init shorthand
    let name = String::from("Alex");
    let age = 27;
    let alex = Person { name, age };

    // Print debug struct
    println!("{:?}", alex);

    // Instantiate a Point
    let point: Point = Point { x: 5.4, y: 7.5 };
    let another_point: Point = Point { x: 2.4, y: 5.3 };

    // Access the fields of the point
    println!("point coordinates: ({}, {})", point.x, point.y);

    // Destructure the point using let binding
    let Point { x: left_edge, y: top_edge } = point;

    let _rectangle = Rectangle {
        // struct instantiation is an expression too
        top_left: Point { x: left_edge, y: top_edge },
        bottom_right: another_point,
    };

    // Instantiate a tuple struct
        let pair = Pair(1, 0.1);
    
    // Access the fields of a tuple struct
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // Destructure a tuple struct
    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);
}

// Output:
// Person { name: "Alex", age: 27 }
// point coordinates: (5.4, 7.5)
// pair contains 1 and 0.1
// pair contains 1 and 0.1
