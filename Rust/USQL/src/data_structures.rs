use crate::compile::Instruction;

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
    pub value: String,
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
            _ => {
                None
            }
        }
    }
}

impl BooleanComparisonOperator {
    pub fn as_str(&self) -> &str {
        match self {
            BooleanComparisonOperator::Or => "|",
            BooleanComparisonOperator::And => "&"
        }
    }


    pub fn from_str(input_str: &str) -> Option<BooleanComparisonOperator> {
        match input_str.to_uppercase().as_str() {
            "|" | "OR" => Option::from(BooleanComparisonOperator::Or),
            "&" | "AND" => Option::from(BooleanComparisonOperator::And),
            _ => {
                None
            }
        }
    }
}


pub type TokenStack = Vec<String>;

pub type InstructionStack = Vec<Instruction>;
