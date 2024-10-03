use std::ops::Add;
use crate::data_structures::{NEGATE_VALUE, CASE_SENS_VALUE, TokenStack};
#[derive(Debug)]
pub enum TokenizeError {}

pub fn tokenize(input_string: &str) -> Result<TokenStack, TokenizeError> {
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
            while !temp_value.is_empty() &&
                (temp_value.chars().next().expect("No Char") == NEGATE_VALUE ||
                    temp_value.chars().next().expect("No Char") == CASE_SENS_VALUE ||
                    temp_value.chars().next().expect("No Char") == '(') {
                if temp_value.chars().next().unwrap() == '(' {
                    result_list.push("(".to_string());
                } else if temp_value.chars().next().unwrap() == CASE_SENS_VALUE {
                    result_list.push(CASE_SENS_VALUE.to_string());
                } else if temp_value.chars().next().unwrap() == NEGATE_VALUE {
                    result_list.push(NEGATE_VALUE.to_string());
                }

                temp_value.remove(0); // Remove the first character
            }
        }

        let mut reversed_stack = Vec::new();
        let mut temp_reversed: String = temp_value.chars().rev().collect();

        while temp_reversed.chars().next().unwrap() == ')' {
            reversed_stack.push(")".to_string());
            temp_reversed.remove(0);
        }

        temp_value = temp_reversed.chars().rev().collect();


        if in_quotes && temp_value.ends_with('"') {
            let temp = &split_string[start_index.unwrap()..=index - 1].join(" ").add(" ").add(&*temp_value);
            let token = &temp[1..temp.len() - 1];
            result_list.push(token.to_string());
            in_quotes = false;
        } else if !in_quotes {
            result_list.push(temp_value);
        }

        if !reversed_stack.is_empty() && !in_quotes {
            result_list.extend(reversed_stack.iter().rev().cloned());
        }
    }

    Ok(result_list)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize() {
        let input_query = r#"artist := Pac & album =: "Against The World""#;
        let tokens = match tokenize(input_query) {
            Ok(result) => result,
            Err(error) => panic!("{:?}", error)
        };

        assert_eq!(tokens, vec![
            "artist".to_string(),
            ":=".to_string(),
            "Pac".to_string(),
            "&".to_string(),
            "album".to_string(),
            "=:".to_string(),
            "Against The World".to_string(),
        ]);
    }

    #[test]
    fn test_tokenize_with_parentheses() {
        let input_query = r#"artist := Pac & (album =: "Against The World" | album =: "Strictly 4")"#;
        let tokens = tokenize(input_query).unwrap();
        assert_eq!(tokens, vec![
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
        ]);
    }

    #[test]
    fn test_tokenize_with_negated_parentheses() {
        let input_query = r#"artist := Pac & !(album =: "Against The World" | album =: "Strictly 4")"#;
        let tokens = tokenize(input_query).unwrap();
        assert_eq!(tokens, vec![
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
        ]);
    }

    #[test]
    fn test_tokenize_with_negated_nested_parentheses() {
        let input_query = r#"artist =: Pac & !(album =: "Against The World" | (album =: "Strictly 4" & title = "I get Around"))"#;
        let tokens = tokenize(input_query).unwrap();
        assert_eq!(tokens, vec![
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
        ]);
    }

    #[test]
    fn test_tokenize_multiple_negates() {
        let input_query = r#"!!!!!!((artist = Pac) & (title !^= "Big Poppa"))"#;
        let tokens = match tokenize(input_query) {
            Ok(result) => result,
            Err(error) => panic!("{:?}", error)
        };
        assert_eq!(tokens, vec![
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
        ]);
    }
}
