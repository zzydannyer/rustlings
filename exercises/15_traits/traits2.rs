trait AppendBar {
    fn append_bar(self) -> Self;
}

// TODO: Implement the trait `AppendBar` for a vector of strings.
// `append_bar` should push the string "Bar" into the vector.
impl AppendBar for Vec<String> {
    fn append_bar(mut self) -> Self {
        self.push("Bar".to_string()); // Append "Bar" as a String
        self
    }
}

fn main() {
    // You can optionally experiment here.
    let mut foox = vec![String::from("Foo")].append_bar();
    assert_eq!(foox.pop().unwrap(), "Bar");
    assert_eq!(foox.pop().unwrap(), "Foo");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_vec_pop_eq_bar() {
        let mut fooo = vec![String::from("Foo")].append_bar();
        assert_eq!(fooo.pop().unwrap(), "Bar");
        assert_eq!(fooo.pop().unwrap(), "Foo");
    }
}
