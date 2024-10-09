mod utils;
use wasm_bindgen::prelude::*;

use fitl::data_structures::TableFormat;
use fitl::table::Table;
use fitl::{compile_query, filter};
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

type JSValueTable = Vec<Value>;

#[wasm_bindgen]
pub fn test_json_input(input_table: JsValue) -> Result<JsValue, JsValue> {
    let table_value = match serde_wasm_bindgen::from_value::<JSValueTable>(input_table) {
        Ok(table_value) => table_value,
        Err(error) => return Err(format_js_error("TableFormat", &format!("{:?}", error))),
    };

    match serde_wasm_bindgen::to_value(&table_value) {
        Ok(result) => Ok(result),
        Err(error) => Err(format_js_error("FilterError", &format!("{:?}", error))),
    }
}

#[wasm_bindgen(js_name = "fitl_filter")]
pub fn fitl_filter(
    input_query: String,
    input_table: JsValue,
    table_format: String,
) -> Result<JsValue, JsValue> {
    let table_format_input = match table_format_from_string(&table_format) {
        Some(result) => result,
        None => {
            return Err(format_js_error(
                "TableCreation",
                &format!("InvalidTableFormat HERE {:?}", &table_format),
            ))
        }
    };

    let table_result = match table_format_input {
        TableFormat::JsonArray => {
            let table_value = match serde_wasm_bindgen::from_value::<JSValueTable>(input_table) {
                Ok(table_value) => table_value,
                Err(error) => return Err(format_js_error("Table Format", &format!("{:?}", error))),
            };
            Table::from_json_array(&table_value)
        }
        _ => {
            return Err(format_js_error(
                "TableCreation",
                &format!("InvalidTableFormat {:?}", &table_format),
            ))
        }
    };

    let table = match table_result {
        Ok(table) => table,
        Err(error) => {
            return Err(format_js_error(
                "TableParsingError",
                &format!("{:?}", error),
            ))
        }
    };

    let compiled_query = match compile_query(&input_query, &table.get_column_names()) {
        Ok(result) => result,
        Err(error) => return Err(format_js_error("CompileError", &format!("{:?}", error))),
    };

    match filter(&compiled_query, &table) {
        Ok(result_table) => {
            Ok(serde_wasm_bindgen::to_value(&result_table.to_json_array()).unwrap())
        }
        Err(error) => Err(format_js_error("FilterError", &format!("{:?}", error))),
    }
}
