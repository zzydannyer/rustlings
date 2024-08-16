// Tests are important to ensure that your code does what you think it should
// do.

fn is_even(n: i64) -> bool {
    n % 2 == 0
}

fn main() {
    // You can optionally experiment here.
    assert!(is_even(18));
}

#[cfg(test)]
mod tests {
    use super::is_even;
    // TODO: Import `is_even`. You can use a wildcard to import everything in
    // the outer module.

    #[test]
    fn you_can_assert() {
        // TODO: Test the function `is_even` with some values.
        assert!(is_even(18));
        assert!(!is_even(17));
    }
}
