use crate::table::ColumnType;
use crate::value_parsers::ParsingError;
use wasm_bindgen::prelude::wasm_bindgen;

#[derive(Debug, PartialEq)]
pub enum ComparisonOperator {
    Equals,
    Contains,
    IsIn,
    LessThan,
    MoreThan,
    LessThanEquals,
    MoreThanEquals,
}

#[derive(Debug, PartialEq, Clone)]
pub enum BooleanComparisonOperator {
    Or,
    And,
}

#[derive(Debug, PartialEq)]
pub struct Operation {
    pub column: String,
    pub operation: ComparisonOperator,
    pub value: ColumnType,
    pub negated: bool,
    pub case_sensitive: bool,
}

pub const NEGATE_VALUE: char = '!';
pub const CASE_SENS_VALUE: char = '^';

impl ComparisonOperator {
    pub fn as_str(&self) -> &str {
        match self {
            ComparisonOperator::Equals => "=",
            ComparisonOperator::Contains => "=:",
            ComparisonOperator::IsIn => ":=",
            ComparisonOperator::LessThan => "<",
            ComparisonOperator::MoreThan => ">",
            ComparisonOperator::LessThanEquals => "<=",
            ComparisonOperator::MoreThanEquals => ">=",
        }
    }

    pub fn from_str(input_str: &str) -> Option<ComparisonOperator> {
        match input_str.to_uppercase().as_str() {
            "=" | "EQUALS" => Option::from(ComparisonOperator::Equals),
            "=:" | "CONTAINS" => Option::from(ComparisonOperator::Contains),
            ":=" | "ISIN" => Option::from(ComparisonOperator::IsIn),
            "<" | "LESSTHAN" => Option::from(ComparisonOperator::LessThan),
            "<=" | "LESSTHANEQUALS" => Option::from(ComparisonOperator::LessThanEquals),
            ">" | "MORESTHAN" => Option::from(ComparisonOperator::MoreThan),
            ">=" | "MORESTHANEQUALS" => Option::from(ComparisonOperator::MoreThanEquals),
            _ => None,
        }
    }
}

impl BooleanComparisonOperator {
    pub fn as_str(&self) -> &str {
        match self {
            BooleanComparisonOperator::Or => "|",
            BooleanComparisonOperator::And => "&",
        }
    }

    pub fn from_str(input_str: &str) -> Option<BooleanComparisonOperator> {
        match input_str.to_uppercase().as_str() {
            "|" | "OR" => Option::from(BooleanComparisonOperator::Or),
            "&" | "AND" => Option::from(BooleanComparisonOperator::And),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Instruction {
    Operation(Operation),
    BoolCompOp(BooleanComparisonOperator),
    Negate(String),
    Parentheses(InstructionStack),
}

pub type TokenStack = Vec<String>;

pub type InstructionStack = Vec<Instruction>;

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub enum TableFormat {
    Custom,
    JsonArray,
}

#[derive(Debug, PartialEq)]
pub enum RuntimeError {
    InvalidOperation(String),
    InvalidColumn(String),
}

#[derive(Debug, Clone)]
pub enum ColumnParsingError {
    ColumnNotFound(String),
}

#[derive(Debug, Clone)]
pub enum TableParsingError {
    ColumnParsingError(ColumnParsingError),
    ParseError(ParsingError),
    CouldNotConvertColumn(String),
}

#[derive(Debug, PartialEq)]
pub enum CompileError {
    InvalidToken(String),
    InvalidColumn(String),
    NoMatchingParenthesis(String),
    NoMatchingQuotes(String),
    NoColumnsDetected,
    NotEnoughTokens(TokenStack),
    ParseError(ParsingError),
    WTFIsThisInput(TokenStack),
}

#[derive(Debug, PartialEq)]
pub enum FITLError {
    CompileError(CompileError),
    RuntimeError(RuntimeError),
}
