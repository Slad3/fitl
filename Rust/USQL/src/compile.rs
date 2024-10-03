use crate::data_structures::{BooleanComparisonOperator, ComparisonOperator, InstructionStack, Operation, TokenStack, CASE_SENS_VALUE, NEGATE_VALUE};


#[derive(Debug, PartialEq)]
pub enum Instruction {
    Operation(Operation),
    BoolCompOp(BooleanComparisonOperator),
    Negate(String),
    Parentheses(InstructionStack),
}

#[derive(Debug, PartialEq)]
pub enum CompileError {
    InvalidToken(String),
    NotEnoughTokens(TokenStack),
    WTFIsThisInput(TokenStack),
}

pub fn compile(tokens: TokenStack) -> Result<InstructionStack, CompileError> {
    let (result_tokens, instruction_stack) = parse_s(tokens.clone())?;

    if result_tokens.len() > 0 {
        return Err(CompileError::WTFIsThisInput(tokens));
    }


    Ok(instruction_stack)
}

fn parse_s(mut temp_tokens: TokenStack) -> Result<(TokenStack, InstructionStack), CompileError> {
    let mut temp_stack: InstructionStack = Vec::new();

    if temp_tokens[0] == NEGATE_VALUE.to_string() {
        let (tokens, result_stack) = parse_neg(temp_tokens.split_off(1))?;
        temp_stack.extend(result_stack);
        temp_tokens = tokens;
    } else if temp_tokens[0] == "(" {
        let (tokens, result_stack) = parse_par(temp_tokens.split_off(1))?;
        temp_stack.push(Instruction::Parentheses(result_stack));
        temp_tokens = tokens;
    } else {
        let (tokens, result_operation) = parse_op(temp_tokens)?;
        temp_stack.push(Instruction::Operation(result_operation));
        temp_tokens = tokens;
    }

    if !temp_tokens.is_empty() {
        if let Some(bool_op_result) = BooleanComparisonOperator::from_str(&temp_tokens[0]) {
            temp_stack.push(Instruction::BoolCompOp(bool_op_result));
            let (tokens, result_stack) = parse_s(temp_tokens.split_off(1))?;
            temp_stack.extend(result_stack);
            temp_tokens = tokens;
        }
    }

    Ok((temp_tokens, temp_stack))
}


fn parse_op(mut temp_tokens: TokenStack) -> Result<(TokenStack, Operation), CompileError> {
    if temp_tokens.len() >= 3 {
        let mut negated = false;
        let mut case_sensitive = false;

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
            Some(op) => op
        };

        let column = temp_tokens[0].clone();
        let value = temp_tokens[1 + compare_op_index].clone();

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


fn parse_par(temp_tokens: TokenStack) -> Result<(TokenStack, InstructionStack), CompileError> {
    let mut par_stack: InstructionStack = Vec::new();


    let (result_tokens, result_stack) = parse_s(temp_tokens)?;
    par_stack.extend(result_stack);

    let mut tokens = result_tokens.clone();
    let mut first_character = &tokens.clone()[0];

    while first_character != ")" {
        let (result_tokens, result_stack) = parse_s(tokens)?;
        par_stack.extend(result_stack);

        tokens = result_tokens.clone();
        first_character = &tokens[0];
    }

    Ok((tokens.split_off(1), par_stack))
}


fn parse_neg(temp_tokens: TokenStack) -> Result<(TokenStack, InstructionStack), CompileError> {
    let mut temp_stack: InstructionStack = Vec::new();
    temp_stack.push(Instruction::Negate(NEGATE_VALUE.to_string()));

    let (tokens, result_stack) = parse_s(temp_tokens)?;
    temp_stack.extend(result_stack);
    Ok((tokens, temp_stack))
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::tokenize::tokenize;


    #[test]
    fn test_compile_simple_success() {
        let input_query = vec!["artist".to_string(), "=:".to_string(), "Pac".to_string()];
        let operation_stack = compile(input_query).unwrap();

        let expected_stack = vec![
            Instruction::Operation(Operation {
                column: "artist".to_string(),
                operation: ComparisonOperator::Contains,
                value: "Pac".to_string(),
                negated: false,
                case_sensitive: false,
            })
        ];

        assert_eq!(operation_stack, expected_stack);
    }

    #[test]
    fn test_compile_negated_case_sensitive_operation() {
        let input_query = vec!["artist".to_string(), "!".to_string(), "^".to_string(), "=".to_string(), "Pac".to_string()];
        let operation_stack = compile(input_query).unwrap();

        let expected_stack = vec![
            Instruction::Operation(Operation {
                column: "artist".to_string(),
                operation: ComparisonOperator::Equals,
                value: "Pac".to_string(),
                negated: true,
                case_sensitive: true,
            })
        ];

        assert_eq!(operation_stack, expected_stack);
    }

    #[test]
    fn test_compile_simple_success_worded() {
        let input_query = tokenize("artist contains Pac").unwrap();
        let operation_stack = compile(input_query).unwrap();

        let expected_stack = vec![
            Instruction::Operation(Operation {
                column: "artist".to_string(),
                operation: ComparisonOperator::Contains,
                value: "Pac".to_string(),
                negated: false,
                case_sensitive: false,
            })
        ];

        assert_eq!(operation_stack, expected_stack);
    }

    #[test]
    fn test_compile_parentheses_success() {
        let input_query = tokenize(r#"artist =: Pac & (album =: "Against The World" | album =: "Strictly 4")"#).unwrap();

        let result = compile(input_query).unwrap();

        let expected_result = vec![
            Instruction::Operation(Operation {
                column: "artist".to_string(),
                operation: ComparisonOperator::Contains,
                value: "Pac".to_string(),
                negated: false,
                case_sensitive: false,
            }),
            Instruction::BoolCompOp(BooleanComparisonOperator::And),
            Instruction::Parentheses(vec![
                Instruction::Operation(Operation {
                    column: "album".to_string(),
                    operation: ComparisonOperator::Contains,
                    value: "Against The World".to_string(),
                    negated: false,
                    case_sensitive: false,
                }),
                Instruction::BoolCompOp(BooleanComparisonOperator::Or),
                Instruction::Operation(Operation {
                    column: "album".to_string(),
                    operation: ComparisonOperator::Contains,
                    value: "Strictly 4".to_string(),
                    negated: false,
                    case_sensitive: false,
                })
            ]),
        ];

        assert_eq!(result, expected_result);
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


        let result = compile(tokens).unwrap();

        let expected_result = vec![
            Instruction::Operation(Operation {
                column: "artist".to_string(),
                operation: ComparisonOperator::IsIn,
                value: "Pac".to_string(),
                negated: false,
                case_sensitive: false,
            }),
            Instruction::BoolCompOp(BooleanComparisonOperator::And),
            Instruction::Negate(NEGATE_VALUE.to_string()),
            Instruction::Parentheses(vec![
                Instruction::Operation(Operation {
                    column: "album".to_string(),
                    operation: ComparisonOperator::Contains,
                    value: "Against The World".to_string(),
                    negated: false,
                    case_sensitive: false,
                }),
                Instruction::BoolCompOp(BooleanComparisonOperator::Or),
                Instruction::Parentheses(vec![
                    Instruction::Operation(Operation {
                        column: "album".to_string(),
                        operation: ComparisonOperator::Contains,
                        value: "Strictly 4".to_string(),
                        negated: false,
                        case_sensitive: false,
                    }),
                    Instruction::BoolCompOp(BooleanComparisonOperator::And),
                    Instruction::Operation(Operation {
                        column: "title".to_string(),
                        operation: ComparisonOperator::Equals,
                        value: "I get Around".to_string(),
                        negated: false,
                        case_sensitive: false,
                    }),
                ])
            ]),
        ];

        assert_eq!(result, expected_result);
    }
}
