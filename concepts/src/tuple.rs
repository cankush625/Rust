// Tuple is a collection of values of different types
// 
// Tuples can be used as function arguments and return values
pub fn reverse(pair: (i32, bool)) -> (bool, i32) {
    let (int_param, bool_param) = pair;

    (bool_param, int_param)
}
