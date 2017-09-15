pub fn square(s: u32) -> u64 {
    if s == 0 || s > 64 {
        panic!("Square must be between 1 and 64")
    }
    2u64.pow(s - 1)
}

pub fn total() -> u64 {
    // Amount of grains on the whole board is 2^64 - 1, 
    // which accidentally happens to be maximum number 
    // you can fit into u64.
    u64::max_value()
}