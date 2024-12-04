mod compile;
pub mod data_structures;
mod filter;
// mod table;
mod tokenize;

mod table;
mod value_parsers;

pub use crate::table::Table;
use crate::tokenize::tokenize;

use crate::compile::compile_tokens;

use crate::filter::filter_table;
use data_structures::CompileError;

pub use crate::data_structures::{FITLError, InstructionStack};
use crate::table::ColumnType;

/// Pre-compiles query, gives specific compile errors for better query linting
///
/// # Arguments
///
/// * `input_query` - Input query as an &str
/// * `table` - Table reference for checking column names
///
/// # Returns
///
/// Instruction stack used for later 'filter' function.
pub fn compile_query(
    input_query: &str,
    columns: &Vec<String>,
    column_types: &Vec<ColumnType>,
) -> Result<InstructionStack, CompileError> {
    let tokens = match tokenize(&input_query) {
        Ok(tokens) => tokens,
        Err(error) => return Err(error),
    };

    match compile_tokens(tokens, columns, column_types) {
        Ok(stack) => Ok(stack),
        Err(error) => Err(error),
    }
}

/// Filters table from with an instruction stack input.
///
/// # Arguments
///
/// * `compiled_query` - Compiled query from 'compile_query'
/// * `table` - Table reference to create filtered table from
///
/// # Returns
///
/// A filtered down version of the inputted Table
pub fn filter(compiled_query: &InstructionStack, table: &Table) -> Result<Table, FITLError> {
    match filter_table(compiled_query, table) {
        Ok(result_table) => Ok(result_table),
        Err(error) => Err(FITLError::RuntimeError(error)),
    }
}

/// Full Table Filters table from with an instruction stack input.
///
/// # Arguments
///
/// * `input_string` - Input query as an &str
/// * `table` - Table reference to create filtered table from
///
/// # Returns
///
/// A filtered down version of the inputted Table
pub fn filter_full(input_string: &str, table: &Table) -> Result<Table, FITLError> {
    let instruction_stack = match compile_query(
        input_string,
        &table.get_column_names(),
        &table.get_column_types(),
    ) {
        Ok(stack) => stack,
        Err(error) => return Err(FITLError::CompileError(error)),
    };

    match filter_table(&instruction_stack, table) {
        Ok(result_table) => Ok(result_table),
        Err(error) => Err(FITLError::RuntimeError(error)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::table::ColumnType;
    use serde_json::json;
    use serde_json::Value;

    fn get_test_json_table() -> Vec<Value> {
        json!([
            {"artist": "2Pac", "album": "Me Against the World", "title": "So Many Tears"},
            {"artist": "2Pac", "album": "Me Against the World", "title": "Lord Knows"},
            {"artist": "2Pac", "album": "All Eyez on Me", "title": "All Eyez on Me"},
            {"artist": "2Pac", "album": "All Eyez on Me", "title": "2 Of Amerikaz Most Wanted"},
            {"artist": "2Pac", "album": "All Eyez on Me", "title": "Heartz of Men"},
            {"artist": "Makaveli", "album": "The Don Killuminati: The 7 Day Theory", "title": "Toss It Up"},
            {"artist": "Makaveli", "album": "The Don Killuminati: The 7 Day Theory", "title": "Me And My Girlfriend"},
            {"artist": "Makaveli", "album": "The Don Killuminati: The 7 Day Theory", "title": "Against All Odds"},
        ]).as_array().unwrap().clone()
    }

    fn get_test_food_json_array() -> Vec<Value> {
        json!([
            {"name": "apple", "category": "fruit", "amount": 42,},
            {"name": "bananas", "category": "fruit", "amount": 3,},
            {"name": "flour", "category": "ingredient", "amount": null,},
            {"name": "flour", "category": "ingredient", "amount": 5.67,},
        ])
        .as_array()
        .unwrap()
        .clone()
    }

    #[test]
    fn test_empty_query() {
        let table: Table = Table::from_json_array(&get_test_json_table()).unwrap();

        // let query = "!(artist =: Pac)".to_string();
        // let query = r#"(artist =: "Pac") | artist =: Makaveli"#.to_string();
        let query = r#"artist =: pac"#.to_string();
        // let query = "!".to_string();

        match filter_full(&query, &table) {
            Ok(result) => println!("{}", result.to_json_array().as_array().unwrap().len()),
            Err(error) => println!("{:?}", error),
        };

        // println!("{:?}", filter_full(&query, &table))
    }

    #[test]
    fn test_numerical_table_comparisons() {
        let mut table: Table = Table::from_json_array(&get_test_food_json_array()).unwrap();

        table
            .set_column_type("amount", ColumnType::Number(None))
            .expect("TODO: panic message");

        for row in &table {
            // println!("{:?}", &row);
            println!("{:?}\t|\t {:?}", &row.get("amount"), &row.get("name"));
        }

        println!("\n\n\n");

        for row in filter(
            &compile_query(
                "amount > -1",
                &table.get_column_names(),
                &table.get_column_types(),
            )
            .unwrap(),
            &table,
        )
        .unwrap()
        {
            println!("{:?}\t|\t {:?}", &row.get("amount"), &row.get("name"));
        }
    }
}
