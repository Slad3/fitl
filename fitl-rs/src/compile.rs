use crate::data_structures::{
    BooleanComparisonOperator, ComparisonOperator, CompileError, Instruction, InstructionStack,
    Operation, TokenStack, CASE_SENS_VALUE, NEGATE_VALUE,
};
use crate::table::ColumnType;
use crate::value_parsers::parse_string_to_number;

pub fn compile_tokens(
    tokens: TokenStack,
    columns: &Vec<String>,
    column_types: &Vec<ColumnType>,
) -> Result<InstructionStack, CompileError> {
    if columns.is_empty() {
        return Err(CompileError::NoColumnsDetected);
    }

    if tokens.is_empty() {
        return Ok(InstructionStack::new());
    }

    let (result_tokens, instruction_stack) = parse_s(tokens.clone(), columns, column_types)?;

    if result_tokens.len() > 0 {
        return Err(CompileError::WTFIsThisInput(tokens));
    }

    Ok(instruction_stack)
}

fn parse_s(
    mut temp_tokens: TokenStack,
    columns: &Vec<String>,
    column_types: &Vec<ColumnType>,
) -> Result<(TokenStack, InstructionStack), CompileError> {
    let mut temp_stack: InstructionStack = Vec::new();

    if !temp_tokens.is_empty() && temp_tokens[0] == NEGATE_VALUE.to_string() {
        let (tokens, result_stack) = parse_neg(temp_tokens.split_off(1), columns, column_types)?;
        temp_stack.extend(result_stack);
        temp_tokens = tokens;
    } else if !temp_tokens.is_empty() && temp_tokens[0] == "(" {
        let (tokens, result_stack) = parse_par(temp_tokens.split_off(1), columns, column_types)?;
        temp_stack.push(Instruction::Parentheses(result_stack));
        temp_tokens = tokens;
    } else if !temp_tokens.is_empty() && columns.contains(&temp_tokens[0]) {
        let (tokens, result_operation) = parse_op(temp_tokens, columns, column_types)?;
        temp_stack.push(Instruction::Operation(result_operation));
        temp_tokens = tokens;
    } else if !temp_tokens.is_empty() && !columns.contains(&temp_tokens[0]) {
        return Err(CompileError::InvalidColumn(temp_tokens[0].clone()));
    }

    if !temp_tokens.is_empty() {
        if let Some(bool_op_result) = BooleanComparisonOperator::from_str(&temp_tokens[0]) {
            temp_stack.push(Instruction::BoolCompOp(bool_op_result));
            let (tokens, result_stack) = parse_s(temp_tokens.split_off(1), columns, column_types)?;
            temp_stack.extend(result_stack);
            temp_tokens = tokens;
        }
    }

    Ok((temp_tokens, temp_stack))
}

fn parse_op(
    mut temp_tokens: TokenStack,
    columns: &Vec<String>,
    column_types: &Vec<ColumnType>,
) -> Result<(TokenStack, Operation), CompileError> {
    if temp_tokens.len() >= 3 {
        let mut negated = false;
        let mut case_sensitive = false;

        let column = temp_tokens[0].clone();

        if !columns.contains(&column) {
            return Err(CompileError::InvalidColumn(column.clone()));
        }

        let mut compare_op_index = 1;

        if temp_tokens[compare_op_index] == NEGATE_VALUE.to_string() {
            negated = true;
            compare_op_index += 1;
        }

        if temp_tokens[compare_op_index] == CASE_SENS_VALUE.to_string() {
            case_sensitive = true;
            compare_op_index += 1;
        }

        let compare_op = &temp_tokens[compare_op_index];

        let operation = match ComparisonOperator::from_str(compare_op) {
            None => return Err(CompileError::InvalidToken(compare_op.to_string())),
            Some(op) => op,
        };

        let value_string = temp_tokens[1 + compare_op_index].clone();

        let column_index = columns
            .iter()
            .position(|c| c == &column)
            .expect("Column not found");
        let column_type = &column_types[column_index];

        let value = match column_type {
            ColumnType::String(_) => ColumnType::String(Some(value_string)),
            ColumnType::Number(_) => {
                let temp = match parse_string_to_number::<f32>(&value_string) {
                    Ok(num) => num,
                    Err(error) => return Err(CompileError::ParseError(error)),
                };
                ColumnType::Number(Some(temp))
            }
            ColumnType::Bool(_) => match value_string.as_str().to_lowercase().as_ref() {
                "true" | "1" => ColumnType::Bool(Some(true)),
                "false" | "0" => ColumnType::Bool(Some(false)),
                _ => {
                    return Err(CompileError::InvalidToken(format!("Column {column} is a boolean column, {value_string} must be either \"true\" or \"false\"")));
                }
            },
        };

        let remaining_tokens = temp_tokens.split_off(2 + compare_op_index);
        let operation = Operation {
            column,
            operation,
            value,
            negated,
            case_sensitive,
        };

        Ok((remaining_tokens, operation))
    } else {
        Err(CompileError::NotEnoughTokens(temp_tokens))
    }
}

fn parse_par(
    temp_tokens: TokenStack,
    columns: &Vec<String>,
    column_types: &Vec<ColumnType>,
) -> Result<(TokenStack, InstructionStack), CompileError> {
    let mut par_stack: InstructionStack = Vec::new();

    let (result_tokens, result_stack) = parse_s(temp_tokens.clone(), columns, column_types)?;
    par_stack.extend(result_stack);

    let mut tokens = result_tokens.clone();
    let mut first_character = &tokens.clone()[0];

    while first_character != ")" {
        let (result_tokens, result_stack) = parse_s(tokens, columns, column_types)?;
        par_stack.extend(result_stack);

        tokens = result_tokens.clone();
        first_character = &tokens[0];
    }

    Ok((tokens.split_off(1), par_stack))
}

fn parse_neg(
    temp_tokens: TokenStack,
    columns: &Vec<String>,
    column_types: &Vec<ColumnType>,
) -> Result<(TokenStack, InstructionStack), CompileError> {
    let mut temp_stack: InstructionStack = Vec::new();
    temp_stack.push(Instruction::Negate(NEGATE_VALUE.to_string()));

    let (tokens, result_stack) = parse_s(temp_tokens, columns, column_types)?;
    temp_stack.extend(result_stack);
    Ok((tokens, temp_stack))
}

#[allow(dead_code)]
#[cfg(test)]
mod tests {
    use super::*;
    use crate::table::ColumnType;
    use crate::tokenize::tokenize;

    fn get_test_columns() -> Vec<String> {
        vec![
            "artist".to_string(),
            "title".to_string(),
            "album".to_string(),
        ]
    }
    fn get_test_column_types() -> Vec<ColumnType> {
        vec![
            ColumnType::String(Some("".to_string())),
            ColumnType::String(Some("".to_string())),
            ColumnType::String(Some("".to_string())),
        ]
    }

    fn get_test_food_columns() -> Vec<String> {
        vec![
            "name".parse().unwrap(),
            "category".to_string(),
            "amount".to_string(),
        ]
    }

    fn get_test_food_column_types() -> Vec<ColumnType> {
        vec![
            ColumnType::String(Some("".parse().unwrap())),
            ColumnType::String(Some("".to_string())),
            ColumnType::Number(Some(0f32)),
        ]
    }

    #[test]
    fn test_compile_simple_success() {
        let input_query = vec!["artist".to_string(), "=:".to_string(), "Pac".to_string()];
        let columns = get_test_columns();
        let column_types = get_test_column_types();
        let operation_stack = compile_tokens(input_query, &columns, &column_types).unwrap();

        let expected_stack = vec![Instruction::Operation(Operation {
            column: "artist".to_string(),
            operation: ComparisonOperator::Contains,
            value: ColumnType::String(Some("Pac".to_string())),
            negated: false,
            case_sensitive: false,
        })];

        assert_eq!(operation_stack, expected_stack);
    }

    #[test]
    fn test_compile_negated_case_sensitive_operation() {
        let input_query = vec![
            "artist".to_string(),
            "!".to_string(),
            "^".to_string(),
            "=".to_string(),
            "Pac".to_string(),
        ];
        let columns = get_test_columns();
        let column_types = get_test_column_types();
        let operation_stack = compile_tokens(input_query, &columns, &column_types).unwrap();

        let expected_stack = vec![Instruction::Operation(Operation {
            column: "artist".to_string(),
            operation: ComparisonOperator::Equals,
            value: ColumnType::String(Some("Pac".to_string())),
            negated: true,
            case_sensitive: true,
        })];

        assert_eq!(operation_stack, expected_stack);
    }

    #[test]
    fn test_compile_simple_success_worded() {
        let input_query = tokenize("artist contains Pac").unwrap();
        let columns = get_test_columns();
        let column_types = get_test_column_types();
        let operation_stack = compile_tokens(input_query, &columns, &column_types).unwrap();

        let expected_stack = vec![Instruction::Operation(Operation {
            column: "artist".to_string(),
            operation: ComparisonOperator::Contains,
            value: ColumnType::String(Some("Pac".to_string())),
            negated: false,
            case_sensitive: false,
        })];

        assert_eq!(operation_stack, expected_stack);
    }

    #[test]
    fn test_compile_parentheses_success() {
        let input_query =
            tokenize(r#"artist =: Pac & (album =: "Against The World" | album =: "Strictly 4")"#)
                .unwrap();
        let columns = get_test_columns();
        let column_types = get_test_column_types();
        let result = compile_tokens(input_query, &columns, &column_types).unwrap();

        let expected_result = vec![
            Instruction::Operation(Operation {
                column: "artist".to_string(),
                operation: ComparisonOperator::Contains,
                value: ColumnType::String(Some("Pac".to_string())),
                negated: false,
                case_sensitive: false,
            }),
            Instruction::BoolCompOp(BooleanComparisonOperator::And),
            Instruction::Parentheses(vec![
                Instruction::Operation(Operation {
                    column: "album".to_string(),
                    operation: ComparisonOperator::Contains,
                    value: ColumnType::String(Some("Against The World".to_string())),
                    negated: false,
                    case_sensitive: false,
                }),
                Instruction::BoolCompOp(BooleanComparisonOperator::Or),
                Instruction::Operation(Operation {
                    column: "album".to_string(),
                    operation: ComparisonOperator::Contains,
                    value: ColumnType::String(Some("Strictly 4".to_string())),
                    negated: false,
                    case_sensitive: false,
                }),
            ]),
        ];

        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_compile_column_error() {
        let input_query = tokenize(r#"badcolumn =: Pac"#).unwrap();
        let columns = get_test_columns();
        let column_types = get_test_column_types();
        let result = compile_tokens(input_query, &columns, &column_types);

        assert!(matches!(result, Err(CompileError::InvalidColumn(_))));
    }

    #[test]
    fn test_compile_asdfasfdcolumn_error() {
        let input_query = tokenize(r#" ((((artist =: Pac)))) "#).unwrap();
        let columns = get_test_columns();
        let column_types = get_test_column_types();
        let result = compile_tokens(input_query, &columns, &column_types);

        match result {
            Err(error) => {
                eprintln!("Error: {:?}", error);
            }
            Ok(_) => {}
        }
    }

    #[test]
    fn test_compile_nested_parentheses_success() {
        let tokens = vec![
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
        ];
        let columns = get_test_columns();
        let column_types = get_test_column_types();
        let result = compile_tokens(tokens, &columns, &column_types).unwrap();

        let expected_result = vec![
            Instruction::Operation(Operation {
                column: "artist".to_string(),
                operation: ComparisonOperator::IsIn,
                value: ColumnType::String(Some("Pac".to_string())),
                negated: false,
                case_sensitive: false,
            }),
            Instruction::BoolCompOp(BooleanComparisonOperator::And),
            Instruction::Negate(NEGATE_VALUE.to_string()),
            Instruction::Parentheses(vec![
                Instruction::Operation(Operation {
                    column: "album".to_string(),
                    operation: ComparisonOperator::Contains,
                    value: ColumnType::String(Some("Against The World".to_string())),
                    negated: false,
                    case_sensitive: false,
                }),
                Instruction::BoolCompOp(BooleanComparisonOperator::Or),
                Instruction::Parentheses(vec![
                    Instruction::Operation(Operation {
                        column: "album".to_string(),
                        operation: ComparisonOperator::Contains,
                        value: ColumnType::String(Some("Strictly 4".to_string())),
                        negated: false,
                        case_sensitive: false,
                    }),
                    Instruction::BoolCompOp(BooleanComparisonOperator::And),
                    Instruction::Operation(Operation {
                        column: "title".to_string(),
                        operation: ComparisonOperator::Equals,
                        value: ColumnType::String(Some("I get Around".to_string())),
                        negated: false,
                        case_sensitive: false,
                    }),
                ]),
            ]),
        ];

        assert_eq!(result, expected_result);
    }
}
