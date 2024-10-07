use crate::data_structures::{
    BooleanComparisonOperator, ComparisonOperator, Instruction, InstructionStack, Operation,
    RuntimeError,
};
use serde_json::{json, Value};

use crate::table::{Row, Table};

fn operate(operation: &Operation, row: &Row) -> Result<bool, RuntimeError> {
    let mut operation_value = operation.value.clone();
    let mut row_value = match row.get(&operation.column) {
        None => {
            return Err(RuntimeError::InvalidColumn(
                format!("Invalid Operation: {}", &operation.column).to_string(),
            ))
        }
        Some(value) => value.clone(),
    };

    if !operation.case_sensitive {
        operation_value = operation_value.to_lowercase();
        row_value = row_value.to_lowercase();
    }

    let result = match operation.operation {
        ComparisonOperator::Equals => operation_value == row_value,
        ComparisonOperator::Contains => row_value.contains(&operation_value),
        ComparisonOperator::IsIn => operation_value.contains(&row_value),
        ComparisonOperator::LessThan => row_value < operation_value,
        ComparisonOperator::LessThanEquals => row_value <= operation_value,
        ComparisonOperator::MoreThan => row_value > operation_value,
        ComparisonOperator::MoreThanEquals => row_value >= operation_value,
    };

    if operation.negated {
        Ok(!result)
    } else {
        Ok(result)
    }
}

fn boolean_operate(
    a: &bool,
    b: &bool,
    boolean_comparison_operator: &BooleanComparisonOperator,
) -> bool {
    match boolean_comparison_operator {
        BooleanComparisonOperator::Or => *a || *b,
        BooleanComparisonOperator::And => *a && *b,
    }
}

fn filter_row(instruction_stack: &InstructionStack, row: &Row) -> Result<bool, RuntimeError> {
    let mut current_bool_value: bool = true;
    let mut boolean_comparison_operator: BooleanComparisonOperator = BooleanComparisonOperator::And;

    for instruction in instruction_stack {
        match instruction {
            Instruction::Negate(_) => current_bool_value = !current_bool_value,
            Instruction::BoolCompOp(bool_operator) => {
                boolean_comparison_operator = bool_operator.clone()
            }
            Instruction::Operation(operation) => {
                let operation_result = operate(&operation, row)?;
                current_bool_value = boolean_operate(
                    &current_bool_value,
                    &operation_result,
                    &boolean_comparison_operator,
                )
            }
            Instruction::Parentheses(parenth_stack) => {
                let operation_result = filter_row(parenth_stack, row)?;
                current_bool_value = boolean_operate(
                    &current_bool_value,
                    &operation_result,
                    &boolean_comparison_operator,
                )
            }
        }
    }

    Ok(current_bool_value)
}

pub fn filter_table(
    instruction_stack: &InstructionStack,
    table: &Table,
) -> Result<Table, RuntimeError> {
    let mut result_rows: Vec<Row> = Vec::new();
    for row in table {
        if filter_row(&instruction_stack, &row)? {
            result_rows.push(row.clone());
        }
    }

    Ok(Table::from_rows(result_rows))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compile::compile_tokens;
    use crate::data_structures::TableParsingError;
    use crate::tokenize::tokenize;

    fn get_test_columns() -> Vec<String> {
        vec![
            "artist".to_string(),
            "title".to_string(),
            "album".to_string(),
        ]
    }

    fn get_test_json_table() -> Value {
        json!([
            {"artist": "2Pac", "album": "Me Against the World", "title": "So Many Tears"},
            {"artist": "2Pac", "album": "Me Against the World", "title": "Lord Knows"},
            {"artist": "2Pac", "album": "All Eyez on Me", "title": "All Eyez on Me"},
            {"artist": "2Pac", "album": "All Eyez on Me", "title": "2 Of Amerikaz Most Wanted"},
            {"artist": "2Pac", "album": "All Eyez on Me", "title": "Heartz of Men"},
            {"artist": "Makaveli", "album": "The Don Killuminati: The 7 Day Theory", "title": "Toss It Up"},
            {"artist": "Makaveli", "album": "The Don Killuminati: The 7 Day Theory", "title": "Me And My Girlfriend"},
            {"artist": "Makaveli", "album": "The Don Killuminati: The 7 Day Theory", "title": "Against All Odds"},
        ])
    }

    fn get_test_table() -> Result<Table, TableParsingError> {
        Table::from_json_array(get_test_json_table())
    }

    fn get_test_row() -> Row {
        get_test_table()
            .unwrap()
            .into_iter()
            .collect::<Vec<_>>()
            .get(0)
            .unwrap()
            .clone()
    }

    #[test]
    fn operate_success() {
        let operation = Operation {
            column: "artist".to_string(),
            operation: ComparisonOperator::Contains,
            value: "Pac".to_string(),
            negated: false,
            case_sensitive: false,
        };
        let row = get_test_row();
        let result = operate(&operation, &row).unwrap();
        assert!(result)
    }

    #[test]
    fn test_filter_row() {
        let row = get_test_row();
        let columns = get_test_columns();

        let input_query = r#"artist =: Pac & album =: "Against the World""#;
        let tokens = tokenize(input_query).unwrap();
        let instruction_stack = compile_tokens(tokens, &columns).unwrap();

        let result = filter_row(&instruction_stack, &row).unwrap();
        assert_eq!(result, true)
    }

    #[test]
    fn test_filter_case_sensitive() {
        let row = get_test_row();
        let columns = get_test_columns();

        let input_query = r#"artist ^=: Pac"#;
        let tokens = tokenize(input_query).unwrap();
        let instruction_stack = compile_tokens(tokens, &columns).unwrap();
        assert_eq!(filter_row(&instruction_stack, &row).unwrap(), true);

        let input_query = r#"artist ^=: pac"#;
        let tokens = tokenize(input_query).unwrap();
        let instruction_stack = compile_tokens(tokens, &columns).unwrap();
        let result = filter_row(&instruction_stack, &row).unwrap();
        assert_eq!(result, false);
    }

    #[test]
    fn test_filter_negate_parenteses() {
        let row = get_test_row();
        let columns = get_test_columns();

        let input_query = r#"!(artist =: Pac)"#;
        let tokens = tokenize(input_query).unwrap();
        let instruction_stack = compile_tokens(tokens, &columns).unwrap();
        assert_eq!(filter_row(&instruction_stack, &row).unwrap(), false);
    }

    #[test]
    fn test_filter_table() {
        let table: Table = get_test_table().unwrap();
        let columns = get_test_columns();

        let input_query = r#"artist = Makaveli"#;
        let tokens = tokenize(input_query).unwrap();
        let instruction_stack = compile_tokens(tokens, &columns).unwrap();
        let result_table = filter_table(&instruction_stack, &table).unwrap();

        let expected_table = Table::from_json_array(json!([
            {"artist": "Makaveli", "album": "The Don Killuminati: The 7 Day Theory", "title": "Toss It Up"},
            {"artist": "Makaveli", "album": "The Don Killuminati: The 7 Day Theory", "title": "Me And My Girlfriend"},
            {"artist": "Makaveli", "album": "The Don Killuminati: The 7 Day Theory", "title": "Against All Odds"},
        ])).unwrap();

        assert_eq!(result_table, expected_table);
    }
}
