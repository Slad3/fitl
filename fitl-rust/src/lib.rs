mod compile;
pub mod data_structures;
mod filter;
// mod table;
mod tokenize;

pub mod table;

use crate::table::Table;
use crate::tokenize::tokenize;

use crate::compile::compile_tokens;
use crate::data_structures::{FITLError, InstructionStack};
use crate::filter::filter_table;
use data_structures::CompileError;

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
) -> Result<InstructionStack, CompileError> {
    let tokens = match tokenize(&input_query) {
        Ok(tokens) => tokens,
        Err(error) => return Err(error),
    };

    match compile_tokens(tokens, columns) {
        Ok(stack) => Ok(stack),
        Err(error) => Err(error),
    }
}

/// Filters table from with an instruction stack input.
///
/// # Arguments
///
/// * `compiled_query` - Compiled query from the 'compile_query' function
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

pub fn filter_full(input_string: &str, table: &Table) -> Result<Table, FITLError> {
    let instruction_stack = match compile_query(input_string, &table.get_column_names()) {
        Ok(stack) => stack,
        Err(error) => return Err(FITLError::CompileError(error)),
    };

    match filter_table(&instruction_stack, table) {
        Ok(result_table) => Ok(result_table),
        Err(error) => Err(FITLError::RuntimeError(error)),
    }
}
