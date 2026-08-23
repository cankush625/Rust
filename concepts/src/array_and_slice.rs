// An Array is a collection of objects of the same type T,
// stored in a contiguous memory.
// Slices are similar to array, but their length is not known
// at the compile time. Instead a slice is a two word object;
// the first word is a pointer to the data and the second word
// is the lenght of the slice.
fn analyze_slice(slice: &[i32]) {
    println!("First element of the slice: {}", slice[0]);
    println!("The slice has {} elements", slice.len());
}

pub fn array_and_slice_usage() {
    // Fixed array size
    let arr_lim: [i32; 5] = [1, 2, 3, 4, 5];
    
    // All elements can be initialized to the same value
    let arr_same: [i32; 200] = [0; 200];

    // Indexing starts at 0
    println!("First element of the array: {}", arr_lim[0]);
    println!("Second element of the array: {}", arr_lim[1]);

    // len() returns the count of elements in the array
    println!("Lenght of the array is: {}", arr_lim.len());

    // Arrays can be automatically borrowed as slices
    println!("Borrow the whole array as a slice.");
    analyze_slice(&arr_lim);

    // Slices can point to a section of an array.
    // They are of the form [start_index..end_index].
    println!("Borrow a section of an array as a slice.");
    analyze_slice(&arr_same[1..4]);

    // Example of empty slice `&[]`
    let empty_array: [u32; 0] = [];
    assert_eq!(&empty_array, &[]);

    // Arrays can be safely accessed using `.get`, which returns an
    // `Option`. This can be matched as shown below, or used with
    // `.expect()` if you would like the program to exit with a nice
    // message instead of happily continue.
    for i in 0..arr_lim.len() + 1 { // Oops, one element too far!
        match arr_lim.get(i) {
            Some(xval) => println!("{}: {}", i, xval),
            None => println!("Slow down! {} is too far!", i),
        }
    }
}

// Output:
// First element of the array: 1
// Second element of the array: 2
// Lenght of the array is: 5
// Borrow the whole array as a slice.
// First element of the slice: 1
// The slice has 5 elements
// Borrow a section of an array as a slice.
// First element of the slice: 0
// The slice has 3 elements
// 0: 1
// 1: 2
// 2: 3
// 3: 4
// 4: 5
// Slow down! 5 is too far!
