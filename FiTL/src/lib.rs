mod compile;
mod data_structures;
mod filter;
mod table;
mod tokenize;

use crate::table::Table;
use crate::tokenize::tokenize;

use crate::compile::compile_tokens;
use crate::data_structures::{FITLError, InstructionStack};
use crate::filter::filter_table;
use data_structures::CompileError;

fn compile_query(input_string: &str, table: &Table) -> Result<InstructionStack, CompileError> {
    let tokens = match tokenize(&input_string) {
        Ok(tokens) => tokens,
        Err(error) => return Err(error),
    };

    match compile_tokens(tokens, &table.get_column_names()) {
        Ok(stack) => Ok(stack),
        Err(error) => Err(error),
    }
}

pub fn filter_from_compiled_query(
    instruction_stack: &InstructionStack,
    table: &Table,
) -> Result<Table, FITLError> {
    match filter_table(instruction_stack, table) {
        Ok(result_table) => Ok(result_table),
        Err(error) => Err(FITLError::RuntimeError(error)),
    }
}

pub fn filter(input_string: &str, table: &Table) -> Result<Table, FITLError> {
    let instruction_stack = match compile_query(input_string, &table) {
        Ok(stack) => stack,
        Err(error) => return Err(FITLError::CompileError(error)),
    };

    match filter_table(&instruction_stack, table) {
        Ok(result_table) => Ok(result_table),
        Err(error) => Err(FITLError::RuntimeError(error)),
    }
}
