mod tuple;
mod array_and_slice;
mod struct_;
mod enum_;
mod enum_use;

fn main() {
    // Tuple module function call
    tuple::reverse((1, true));

    // Array and Slice module function call
    array_and_slice::array_and_slice_usage();

    // Struct module function call
    struct_::execute();

    // Enum module function call
    enum_::execute();
    enum_use::execute();
}
