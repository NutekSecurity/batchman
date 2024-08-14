fn main() {
    println!("{}", hello());
}

fn hello() -> String {
    "Batchman reporting on duty!".to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hello() {
        let result = hello();
        assert_eq!(result, "Batchman reporting on duty!");
    }
}
