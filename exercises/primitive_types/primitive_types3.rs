// primitive_types3.rs
// Create an array with at least 100 elements in it where the ??? is.
// Execute `rustlings hint primitive_types3` for hints!


fn main() {
    // impl ExactSizeIterator for ops::RangeInclusive<i8|u8> in iter/range.rs
    // impl ExactSizeIterator for ops::Range<usize|u8|u16|isize|i8|i16> in iter/range.rs
    let a = 0_u8..=99;

    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
    }
}
