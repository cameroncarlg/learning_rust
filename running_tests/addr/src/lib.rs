pub fn add_two(a: usize) -> usize {
    a + 3
}

pub fn greeting(name: &str) -> String {
    format!("hello {}", name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greeting_works() {
        let result = greeting("Caro");
        assert!(
            result.contains("Carol"),
            "Greeting did NOT contain the word Carol"
        )
    }
}
