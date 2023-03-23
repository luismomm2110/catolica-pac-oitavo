use std::num::ParseIntError;

pub fn factorial(n: Result<i32, ParseIntError>) -> u32 {
    match n {
        0 => 1,
        1 => 1,
        _ => n * factorial(n - 1),
    }
}