use crate::data_structures::{
    CompileError, ParenthesesMatch, TokenStack, CASE_SENS_VALUE, NEGATE_VALUE,
};
use std::ops::Add;

#[allow(dead_code)]
fn check_parenthesis(input: &TokenStack) -> ParenthesesMatch {
    let mut left: usize = 0;
    let mut right: usize = 0;

    for token in input {
        match token.as_str() {
            "(" => left += 1,
            ")" => right += 1,
            _ => {}
        }
    }

    if left == right {
        ParenthesesMatch::True
    } else if left >= right {
        ParenthesesMatch::TooManyLeft(left - right)
    } else {
        ParenthesesMatch::TooManyRight(right - left)
    }
}

fn combine_quotes(input_string: &str) -> Result<Vec<String>, CompileError> {
    let mut result_vec: Vec<String> = Vec::new();

    let mut temp_input = input_string;

    let mut first_index: Option<usize> = temp_input.find('"');

    while first_index.is_some() {
        let second_index = &temp_input[first_index.unwrap() + 1..].find('"');

        if second_index.is_none() {
            return Err(CompileError::NoMatchingQuotes(temp_input.parse().unwrap()));
        }

        let before = &temp_input[..first_index.unwrap()];

        let token =
            &temp_input[first_index.unwrap() + 1..first_index.unwrap() + 1 + second_index.unwrap()];

        let after = &temp_input[first_index.unwrap() + second_index.unwrap() + 2..];

        result_vec.extend(tokenize_sub_string(before)?);
        result_vec.push(token.to_string());
        temp_input = after;

        first_index = temp_input.find('"');
    }

    if !temp_input.is_empty() {
        result_vec.extend(tokenize_sub_string(temp_input)?);
    }

    Ok(result_vec)
}

fn tokenize_sub_string(input_string: &str) -> Result<TokenStack, CompileError> {
    if input_string.is_empty() {
        return Ok(TokenStack::new());
    }
    let mut result_list: TokenStack = Vec::new();

    let split_string: Vec<&str> = input_string.split_whitespace().collect();

    let mut start_index = None;
    let mut in_quotes = false;

    for (index, &value) in split_string.iter().enumerate() {
        if value.starts_with('"') {
            start_index = Some(index);
            in_quotes = true;
        }

        let mut temp_value = value.to_string();

        if !in_quotes {
            loop {
                match temp_value.chars().next().unwrap_or('a') {
                    '(' => {
                        result_list.push("(".to_string());
                        temp_value.remove(0);
                    }
                    CASE_SENS_VALUE => {
                        result_list.push(CASE_SENS_VALUE.to_string());
                        temp_value.remove(0);
                    }
                    NEGATE_VALUE => {
                        result_list.push(NEGATE_VALUE.to_string());
                        temp_value.remove(0);
                    }
                    _ => break,
                }
            }
        }

        let mut reversed_stack = Vec::new();
        let mut temp_reversed: String = temp_value.chars().rev().collect();

        while temp_reversed.chars().next().unwrap_or('a') == ')' {
            reversed_stack.push(")".to_string());
            temp_reversed.remove(0);
        }

        temp_value = temp_reversed.chars().rev().collect();

        if in_quotes && temp_value.ends_with('"') {
            let token: String;

            if start_index.unwrap() == index {
                token = temp_value[1..temp_value.len() - 1].to_string();
            } else {
                let temp = split_string[start_index.unwrap()..=index - 1]
                    .join(" ")
                    .add(" ")
                    .add(&*temp_value)
                    .clone();

                token = temp[1..temp.len() - 1].to_string();
            }

            result_list.push(token);
            in_quotes = false;
        } else if !in_quotes {
            result_list.push(temp_value);
        }

        if !reversed_stack.is_empty() && !in_quotes {
            result_list.extend(reversed_stack.iter().rev().cloned());
        }
    }

    result_list.retain(|s| !s.is_empty());

    Ok(result_list)
}

pub fn tokenize(input_string: &str) -> Result<TokenStack, CompileError> {
    let tokens = combine_quotes(input_string)?;

    match check_parenthesis(&tokens) {
        ParenthesesMatch::TooManyLeft(amount) => {
            return Err(CompileError::NoMatchingParenthesis(format!(
                "{amount} too many '(' parentheses"
            )))
        }
        ParenthesesMatch::TooManyRight(amount) => {
            return Err(CompileError::NoMatchingParenthesis(format!(
                "{amount} too many ')' parentheses"
            )))
        }
        ParenthesesMatch::True => {}
    }

    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize() {
        let input_query = r#"artist := Pac & album =: "Against The World""#;
        let tokens = match tokenize(input_query) {
            Ok(result) => result,
            Err(error) => panic!("{:?}", error),
        };

        assert_eq!(
            tokens,
            vec![
                "artist".to_string(),
                ":=".to_string(),
                "Pac".to_string(),
                "&".to_string(),
                "album".to_string(),
                "=:".to_string(),
                "Against The World".to_string(),
            ]
        );
    }

    #[test]
    fn test_tokenize_with_parentheses() {
        let input_query =
            r#"artist := Pac & (album =: "Against The World" | album =: "Strictly 4")"#;
        let tokens = tokenize(input_query).unwrap();
        assert_eq!(
            tokens,
            vec![
                "artist".to_string(),
                ":=".to_string(),
                "Pac".to_string(),
                "&".to_string(),
                "(".to_string(),
                "album".to_string(),
                "=:".to_string(),
                "Against The World".to_string(),
                "|".to_string(),
                "album".to_string(),
                "=:".to_string(),
                "Strictly 4".to_string(),
                ")".to_string(),
            ]
        );
    }

    #[test]
    fn test_tokenize_with_negated_parentheses() {
        let input_query =
            r#"artist := Pac & !(album =: "Against The World" | album =: "Strictly 4")"#;
        let tokens = tokenize(input_query).unwrap();
        assert_eq!(
            tokens,
            vec![
                "artist".to_string(),
                ":=".to_string(),
                "Pac".to_string(),
                "&".to_string(),
                "!".to_string(),
                "(".to_string(),
                "album".to_string(),
                "=:".to_string(),
                "Against The World".to_string(),
                "|".to_string(),
                "album".to_string(),
                "=:".to_string(),
                "Strictly 4".to_string(),
                ")".to_string(),
            ]
        );
    }

    #[test]
    fn test_tokenize_with_negated_nested_parentheses() {
        let input_query = r#"artist =: Pac & !(album =: "Against The World" | (album =: "Strictly 4" & title = "I get Around"))"#;
        let tokens = tokenize(input_query).unwrap();
        assert_eq!(
            tokens,
            vec![
                "artist".to_string(),
                "=:".to_string(),
                "Pac".to_string(),
                "&".to_string(),
                "!".to_string(),
                "(".to_string(),
                "album".to_string(),
                "=:".to_string(),
                "Against The World".to_string(),
                "|".to_string(),
                "(".to_string(),
                "album".to_string(),
                "=:".to_string(),
                "Strictly 4".to_string(),
                "&".to_string(),
                "title".to_string(),
                "=".to_string(),
                "I get Around".to_string(),
                ")".to_string(),
                ")".to_string(),
            ]
        );
    }

    #[test]
    fn test_tokenize_multiple_negates() {
        let input_query = r#"!!!!!!((artist = Pac) & (title !^= "Big Poppa"))"#;
        let tokens = match tokenize(input_query) {
            Ok(result) => result,
            Err(error) => panic!("{:?}", error),
        };
        assert_eq!(
            tokens,
            vec![
                "!".to_string(),
                "!".to_string(),
                "!".to_string(),
                "!".to_string(),
                "!".to_string(),
                "!".to_string(),
                "(".to_string(),
                "(".to_string(),
                "artist".to_string(),
                "=".to_string(),
                "Pac".to_string(),
                ")".to_string(),
                "&".to_string(),
                "(".to_string(),
                "title".to_string(),
                "!".to_string(),
                "^".to_string(),
                "=".to_string(),
                "Big Poppa".to_string(),
                ")".to_string(),
                ")".to_string(),
            ]
        );
    }

    #[test]
    fn test_tokenize_quotes_success() {
        let input_query = r#"artist = "The Roots""#;
        let tokens = combine_quotes(input_query).unwrap();
        assert_eq!(tokens, vec!["artist", "=", "The Roots"])
    }

    #[test]
    fn test_tokenize_quotes_complex_success() {
        let input_query = r#"!!!!!!((artist = Pac) & (title !^= "Big Poppa"))"#;
        let tokens = combine_quotes(input_query);
        tokens.unwrap();
        // assert!(tokens.is_ok());
    }

    #[test]
    fn test_tokenize_quotes_complex_fail() {
        let input_query = r#"!!!!!!((artist = Pac) & (title !^= "Big Poppa))"#;
        let tokens = combine_quotes(input_query);

        assert!(tokens.is_err());
    }
}
