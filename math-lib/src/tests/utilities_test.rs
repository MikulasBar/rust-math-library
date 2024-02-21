
#[cfg(test)]
mod tests {
    use crate::utilities::parser_utils::*;

    #[test]
    fn test_split_surface() {
        let string = "45 + 2*(78 + (x + y) + 7) / 2 + 2";
        let vec = split_surface(string, '+');
        let expected = vec!["45", "2*(78+(x+y)+7)/2", "2"];
        let expected: Vec<String> = expected.into_iter()
            .map(|x| x.to_string())
            .collect();

        assert_eq!(vec, expected);
    }
}