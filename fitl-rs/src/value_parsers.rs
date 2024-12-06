use num_traits::Num;
use std::str::FromStr;

#[derive(Debug, PartialEq, Clone)]
pub enum ParsingError {
    CouldNotParse(String),
}

pub fn parse_string_to_number<T>(input: &str) -> Result<T, ParsingError>
where
    T: Num + FromStr,
{
    input
        .parse::<T>()
        .map_err(|_| ParsingError::CouldNotParse(input.to_string()))
}

pub fn parse_string_to_bool(input: &str) -> Result<bool, ParsingError> {
    input
        .parse::<bool>()
        .map_err(|_| ParsingError::CouldNotParse(input.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_string_to_number_success() {
        let result = parse_string_to_number::<i32>("645").unwrap();
        let expected: i32 = 645;
        assert_eq!(result, expected);

        let result = parse_string_to_number::<f64>("645").unwrap();
        let expected: f64 = 645f64;
        assert_eq!(result, expected);
    }
}
