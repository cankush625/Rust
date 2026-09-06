fn freezing() {
    // When data is bound by the same name immutability, it also freezes.
    // Frozen data can't be modified until the immutable binding goes
    // out of scope.
    let mut _mutable_integer = 7i32;

    {
        // Shadowing by immutable `_mutable_integer`
        let _mutable_integer = _mutable_integer;

        // Below line will raise error since `_mutable_integer` is frozen
        // in this scope
        // _mutable_integer = 2;

        // `_mutable_integer` goes out of scope
    }

    _mutable_integer = 3;
}

pub fn execute() {
    freezing();
}
