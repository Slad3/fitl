use crate::data_structures::{ColumnParsingError, TableFormat, TableParsingError};
use crate::value_parsers::{parse_string_to_bool, parse_string_to_number};
use serde_json::{Map, Value};
use std::cmp::PartialEq;
use std::collections::HashMap;

pub type Row = HashMap<String, ColumnType>;
pub type Columns = HashMap<String, Vec<ColumnType>>;

#[derive(Debug, Clone, PartialEq)]
pub enum ColumnType {
    String(Option<String>),
    Number(Option<f32>),
    Bool(Option<bool>),
}

impl ColumnType {
    pub fn to_string(&self) -> String {
        match self {
            ColumnType::String(val) => val.clone().unwrap_or("null".to_string()),
            ColumnType::Number(val) => match val.clone() {
                Some(n) => n.to_string(),
                None => "null".to_string(),
            },
            ColumnType::Bool(val) => match val.clone() {
                Some(n) => n.to_string(),
                None => "null".to_string(),
            },
        }
    }

    pub fn to_value(&self) -> Option<String> {
        match self {
            ColumnType::String(val) => match val.clone() {
                Some(n) => Some(n),
                None => None,
            },
            ColumnType::Number(val) => match val.clone() {
                Some(n) => Some(n.to_string()),
                None => None,
            },
            ColumnType::Bool(val) => match val.clone() {
                Some(n) => Some(n.to_string()),
                None => None,
            },
        }
    }
}

#[derive(Debug, Clone)]
pub struct Table {
    columns: Columns,
    original_format: TableFormat,
    current_index: usize,
}

impl Table {
    pub fn new(columns: Columns, original_format: TableFormat) -> Table {
        Table {
            columns,
            original_format,
            current_index: 0,
        }
    }

    pub fn from_json_array(input_json_array: &Vec<Value>) -> Result<Table, TableParsingError> {
        let mut result_hash: Columns = HashMap::new();

        for json_row in input_json_array.iter() {
            for (key, value) in json_row.as_object().expect("Unable to unwrap Value").iter() {
                let column_array = result_hash.entry(key.clone()).or_insert_with(Vec::new);

                let result: ColumnType = match value {
                    Value::Null => ColumnType::String(None),
                    Value::String(val) => ColumnType::String(Some(val.to_string())),
                    Value::Number(val) => ColumnType::String(Some(val.to_string())),
                    Value::Array(val) => ColumnType::String(Some(format!("{:?}", val))),
                    Value::Object(val) => ColumnType::String(Some(format!("{:?}", val))),
                    Value::Bool(val) => ColumnType::String(Some(val.to_string())),
                };

                column_array.push(result);
            }
        }

        Ok(Table {
            columns: result_hash,
            original_format: TableFormat::JsonArray,
            current_index: 0,
        })
    }

    pub fn to_json_array(&self) -> Value {
        let mut result: Vec<Value> = Vec::new();

        for row in self {
            let mut json_map: Map<String, Value> = Map::new();
            for (key, value) in row {
                match value.to_value() {
                    Some(n) => json_map.insert(key, Value::String(n)),
                    None => json_map.insert(key.to_string(), Value::Null),
                };
            }
            result.push(Value::Object(json_map));
        }

        Value::Array(result)
    }

    pub fn from_rows(rows: Vec<Row>, table_format: &TableFormat) -> Table {
        let mut result_hash: Columns = HashMap::new();

        for row in rows {
            for (key, value) in row.iter() {
                let column_array = result_hash.entry(key.clone()).or_insert_with(Vec::new);
                column_array.push(value.clone());
            }
        }

        Table {
            columns: result_hash,
            current_index: 0,
            original_format: table_format.clone(),
        }
    }

    pub fn to_rows(&self) -> Vec<Row> {
        let mut result_rows: Vec<Row> = Vec::new();

        for row in self {
            result_rows.push(row)
        }

        result_rows
    }

    pub fn get_column_names(&self) -> Vec<String> {
        let mut column_names: Vec<String> = self.columns.keys().cloned().collect();
        column_names.sort();
        column_names
    }

    pub fn get_column_types(&self) -> Vec<ColumnType> {
        let mut column_names: Vec<String> = self.columns.keys().cloned().collect();
        column_names.sort();

        column_names
            .iter()
            .filter_map(|name| {
                self.columns
                    .get(name)
                    .and_then(|column_values| column_values.get(0).cloned())
            })
            .collect()
    }

    pub fn get_original_format(&self) -> &TableFormat {
        &self.original_format
    }

    pub fn set_column_type(
        &mut self,
        column_name: &str,
        column_type: ColumnType,
    ) -> Result<&Table, TableParsingError> {
        let new_column: &mut Vec<ColumnType> = &mut Vec::new();

        for mut row in self.into_iter() {
            if let ColumnType::String(column_value) = row.get_mut(column_name).ok_or_else(|| {
                TableParsingError::ColumnParsingError(ColumnParsingError::ColumnNotFound(format!(
                    "Column not found {column_name}"
                )))
            })? {
                match column_type {
                    ColumnType::Number(_) => {
                        new_column.push(ColumnType::Number(match column_value {
                            Some(column_value) => {
                                match parse_string_to_number::<f32>(column_value.as_str()) {
                                    Ok(n) => Some(n),
                                    Err(error) => return Err(TableParsingError::ParseError(error)),
                                }
                            }
                            None => None,
                        }))
                    }
                    ColumnType::String(_) => {
                        new_column.push(ColumnType::String(column_value.clone()))
                    }
                    ColumnType::Bool(_) => new_column.push(ColumnType::Bool(match column_value {
                        Some(column_value) => match parse_string_to_bool(column_value.as_str()) {
                            Ok(val) => Some(val),
                            Err(error) => return Err(TableParsingError::ParseError(error)),
                        },
                        None => None,
                    })),
                }
            }
        }

        let original_col = self.columns.get_mut(column_name).ok_or_else(|| {
            TableParsingError::ColumnParsingError(ColumnParsingError::ColumnNotFound(format!(
                "Column not found {column_name}"
            )))
        })?;
        *original_col = new_column.clone();

        Ok(self)
    }
}

impl Iterator for Table {
    type Item = Row;

    fn next(&mut self) -> Option<Self::Item> {
        let mut result: Self::Item = HashMap::new();

        let mut changed = false;

        for column_name in self.columns.keys().into_iter() {
            let value = match self
                .columns
                .get(column_name)
                .expect("Column Name doesn't exist")
                .get(self.current_index)
            {
                None => &ColumnType::String(Some("".to_string())),
                Some(val) => {
                    changed = true;
                    &val.clone()
                }
            };

            result.insert(column_name.clone(), value.clone());
        }

        self.current_index += 1;

        if changed {
            Some(result)
        } else {
            None
        }
    }
}

impl PartialEq for Table {
    fn eq(&self, other: &Self) -> bool {
        self.columns == other.columns
    }
    fn ne(&self, other: &Self) -> bool {
        self.columns != other.columns
    }
}

impl<'a> IntoIterator for &'a Table {
    type Item = Row;
    type IntoIter = TableIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        TableIterator {
            table: self,
            current_index: 0,
        }
    }
}

pub struct TableIterator<'a> {
    table: &'a Table,
    current_index: usize,
}

impl<'a> Iterator for TableIterator<'a> {
    type Item = Row;

    fn next(&mut self) -> Option<Self::Item> {
        let mut result: Row = HashMap::new();
        let mut changed = false;

        for column_name in self.table.columns.keys() {
            let value: &ColumnType =
                match self.table.columns.get(column_name)?.get(self.current_index) {
                    None => &ColumnType::String(Some("".to_string())),
                    Some(val) => {
                        changed = true;
                        val
                    }
                };
            result.insert(column_name.clone(), value.clone());
        }

        self.current_index += 1;

        if changed {
            Some(result)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn get_test_json_array() -> Vec<Value> {
        json!([
            {"name": "apple", "category": "fruit", "amount": 42,},
            {"name": "bananas", "category": "fruit", "amount": 3,},
            {"name": "flour", "category": "ingredient", "amount": 15,},
            {"name": "flour", "category": "ingredient", "amount": 5.67,},

        ])
        .as_array()
        .unwrap()
        .clone()
    }

    #[test]
    fn test_table() {
        let table: Table = Table::from_json_array(&get_test_json_array()).unwrap();
        assert_eq!(table.get_column_names(), vec!["amount", "category", "name"]);
        assert_eq!(
            table.get_column_types(),
            vec![
                ColumnType::String(Option::from("42".to_string())),
                ColumnType::String(Option::from("fruit".to_string())),
                ColumnType::String(Option::from("apple".to_string())),
            ]
        );
    }

    #[test]
    fn set_column_type_string_to_number() {
        let mut table: Table = Table::from_json_array(&get_test_json_array()).unwrap();
        assert_eq!(
            table.get_column_types(),
            vec![
                ColumnType::String(Option::from("42".to_string())),
                ColumnType::String(Option::from("fruit".to_string())),
                ColumnType::String(Option::from("apple".to_string())),
            ]
        );

        table
            .set_column_type("amount", ColumnType::Number(None))
            .unwrap();

        assert_eq!(
            table.get_column_types(),
            vec![
                ColumnType::Number(Option::from(42f32)),
                ColumnType::String(Option::from("fruit".to_string())),
                ColumnType::String(Option::from("apple".to_string())),
            ]
        );
    }
}
