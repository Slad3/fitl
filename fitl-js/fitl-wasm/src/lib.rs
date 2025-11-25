#[macro_use]
mod utils;

use std::collections::HashMap;
use std::iter::zip;
use wasm_bindgen::prelude::*;

use fitl::data_structures::{ColumnParsingError, TableFormat, TableParsingError};
use fitl::{compile_query, filter};
use fitl::{ColumnType, Table};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use serde_wasm_bindgen;
use wasm_bindgen::JsValue;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

fn format_js_error(level: &str, details: &String) -> JsValue {
    serde_wasm_bindgen::to_value(&json!({
        "error": {
        "level": level,
        "details": details
    }}))
    .unwrap()
}

fn table_format_from_string(table_format_input: &String) -> Option<TableFormat> {
    match table_format_input.to_uppercase().as_str() {
        "JSARRAY" | "JAVASCRIPTARRAY" => Some(TableFormat::JsonArray),
        &_ => None,
    }
}

fn js_column_type_to_column_type(column_type: &String) -> Option<ColumnType> {
    match column_type.to_lowercase().as_str() {
        "string" => Option::from(ColumnType::String(Option::from(
            "<EmptyString>".to_string(),
        ))),
        "bool" | "boolean" => Option::from(ColumnType::Bool(Option::from(false))),
        "number" | "num" | "double" => Option::from(ColumnType::Number(Option::from(-1f32))),
        _ => None,
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Options {
    #[serde(rename = "tableFormat")]
    table_format: String,
    #[serde(rename = "columnTypes")]
    column_types: HashMap<String, String>,
}

fn column_types_from_options(
    column_types_table: &HashMap<String, String>,
    columns: &Vec<String>,
) -> Result<Vec<ColumnType>, TableParsingError> {
    let mut result: Vec<ColumnType> = Vec::new();

    for (key, _) in column_types_table {
        if !columns.contains(key) {
            return Err(TableParsingError::ColumnParsingError(
                ColumnParsingError::ColumnNotFound(key.clone()),
            ));
        }
    }

    for column_name in columns {
        match column_types_table.get_key_value(column_name) {
            Some((key, value)) => {
                result.push(js_column_type_to_column_type(value).ok_or_else(|| {
                    return TableParsingError::CouldNotConvertColumn(key.clone());
                })?)
            }
            None => {
                result.push(ColumnType::String(Option::from(
                    "<EmptyString>".to_string(),
                )));
            }
        };
    }

    Ok(result)
}

type JSValueTable = Vec<Value>;

#[wasm_bindgen]
pub fn test_json_input(input_table: JsValue) -> Result<JsValue, JsValue> {
    let table_value = match serde_wasm_bindgen::from_value::<JSValueTable>(input_table) {
        Ok(table_value) => table_value,
        Err(error) => {
            return Err(format_js_error("TableFormat", &format!("{:?}", error)));
        }
    };

    match serde_wasm_bindgen::to_value(&table_value) {
        Ok(result) => Ok(result),
        Err(error) => Err(format_js_error("FilterError", &format!("{:?}", error))),
    }
}

#[wasm_bindgen]
pub fn check_syntax(
    query: String,
    columns: Vec<String>,
    options: JsValue,
) -> Result<bool, JsValue> {
    let input_query = query.clone().trim().to_string();

    let options_parsed = match serde_wasm_bindgen::from_value::<Options>(options) {
        Ok(options_parsed) => options_parsed,
        Err(error) => {
            return Err(format_js_error(
                "Options",
                &format!("Unable to parse options {}", error),
            ))
        }
    };

    let column_types_converted =
        match column_types_from_options(&options_parsed.column_types, &columns) {
            Ok(column_types) => column_types,
            Err(error) => return Err(format_js_error("TableError", &format!("{:?}", error))),
        };

    match compile_query(&input_query, &columns, &column_types_converted) {
        Ok(_) => Ok(true),
        Err(error) => Err(format_js_error("CompileError", &format!("{:?}", error))),
    }
}

#[wasm_bindgen(js_name = "fitl_filter_custom_table_format")]
pub fn fitl_filter_custom(
    query: String,
    input_table: JsValue,
    options: JsValue,
) -> Result<JsValue, JsValue> {
    let options_parsed = match serde_wasm_bindgen::from_value::<Options>(options) {
        Ok(options_parsed) => options_parsed,
        Err(error) => {
            return Err(format_js_error(
                "Options",
                &format!("Unable to parse options {}", error),
            ))
        }
    };

    let table_format_input = match table_format_from_string(&options_parsed.table_format) {
        Some(result) => result,
        None => {
            return Err(format_js_error(
                "TableCreation",
                &format!("InvalidTableFormat HERE {:?}", &options_parsed.table_format),
            ));
        }
    };

    let table_result = match table_format_input {
        TableFormat::JsonArray => {
            let table_value = match serde_wasm_bindgen::from_value::<JSValueTable>(input_table) {
                Ok(table_value) => table_value,
                Err(error) => {
                    return Err(format_js_error("Table Format", &format!("{:?}", error)));
                }
            };
            Table::from_json_array(&table_value)
        }
        _ => {
            return Err(format_js_error(
                "TableCreation",
                &format!("InvalidTableFormat {:?}", &options_parsed.table_format),
            ));
        }
    };

    let mut table = match table_result {
        Ok(table) => table,
        Err(error) => {
            return Err(format_js_error(
                "TableParsingError",
                &format!("{:?}", error),
            ));
        }
    };

    let column_types_converted =
        match column_types_from_options(&options_parsed.column_types, &table.get_column_names()) {
            Ok(column_types) => column_types,
            Err(error) => return Err(format_js_error("TableError", &format!("{:?}", error))),
        };

    for (column_type, column) in zip(&column_types_converted, &table.get_column_names()) {
        let column_converted_result = match column_type {
            ColumnType::Number(_) => {
                table.set_column_type(&column, ColumnType::Number(Option::from(-1f32)))
            }
            ColumnType::Bool(_) => {
                table.set_column_type(&column, ColumnType::Bool(Option::from(false)))
            }
            ColumnType::String(_) => Ok(&table),
            ColumnType::Array(_) => Ok(&table),
        };

        if column_converted_result.is_err() {
            return Err(format_js_error(
                "TableError",
                &format!("Unable to convert column {} to {:?}", column, column_type),
            ));
        }
    }

    let input_query = query.clone().trim().to_string();

    let compiled_query = match compile_query(
        &input_query,
        &table.get_column_names(),
        &table.get_column_types(),
    ) {
        Ok(result) => result,
        Err(error) => {
            return Err(format_js_error("CompileError", &format!("{:?}", error)));
        }
    };

    match filter(&compiled_query, &table) {
        Ok(result_table) => {
            Ok(serde_wasm_bindgen::to_value(&result_table.to_json_array()).unwrap())
        }
        Err(error) => Err(format_js_error("FilterError", &format!("{:?}", error))),
    }
}
