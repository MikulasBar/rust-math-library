/// Used to parse `String` to structure like function <br>
/// Use the `functions` module from this library
pub mod parser {
    use regex::*;
    use once_cell::sync::Lazy;

    static FUNC_REGEX: Lazy<Regex> = Lazy::new(||
        Regex::new(r"(\w+)\((.*)\)").unwrap()
    );

    fn testing_function(str: impl Into<String>) {
        let mut string = str.into();
    }
}


#[cfg(test)]
mod tests {

}