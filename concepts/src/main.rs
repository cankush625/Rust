mod tuple;
mod array_and_slice;
mod struct_;

fn main() {
    // Tuple module function calls
    tuple::reverse((1, true));

    // Array and Slice module function call
    array_and_slice::array_and_slice_usage();

    // Struct module function call
    struct_::execute();
}
