use crate::data_structures::TableParsingError;

use num_traits::Num;
use std::str::FromStr;

pub fn parse_string_to_number<T>(input: &str) -> Result<T, TableParsingError>
where
    T: Num + FromStr,
{
    input
        .parse::<T>()
        .map_err(|_| TableParsingError::ParseError(input.to_string()))
}

pub fn parse_string_to_datetime(input: &str) -> Result<String, TableParsingError> {
    Ok(input.to_string())
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
