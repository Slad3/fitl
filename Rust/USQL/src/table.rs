use std::fmt::Write;
use std::collections::HashMap;
use std::fmt::{Display};
use std::hash::Hash;
use std::str::FromStr;
use serde_json::{json, Value};

pub type Row = HashMap<String, String>;

#[derive(Debug, Clone)]
pub struct Table {
    columns: HashMap<String, Vec<String>>,
    current_index: usize,
}


impl Table {
    pub fn new(columns: HashMap<String, Vec<String>>) -> Table {
        Table {
            columns,
            current_index: 0,
        }
    }

    pub fn from_json_array(input_json_array: Value) -> Table {
        let mut result_hash: HashMap<String, Vec<String>> = HashMap::new();

        for json_row in input_json_array.as_array().unwrap().iter() {
            for (key, value) in json_row.as_object().expect("Unable to unwrap Value").iter() {
                let column_array = result_hash.entry(key.clone()).or_insert_with(Vec::new);
                column_array.push(value.as_str().unwrap_or("INVALIDTYPE").to_string());
            }
        }

        Table {
            columns: result_hash,
            current_index: 0,
        }
    }

    pub fn from_rows(rows: Vec<Row>) -> Table {
        let mut result_hash: HashMap<String, Vec<String>> = HashMap::new();

        for row in rows {
            for (key, value) in row.iter() {
                let column_array = result_hash.entry(key.clone()).or_insert_with(Vec::new);
                column_array.push(value.clone());
            }
        }

        Table {
            columns: result_hash,
            current_index: 0,
        }
    }

    pub fn get_column_names(&self) -> Vec<String> {
        let mut result: Vec<String> = self.columns.keys().cloned().collect();
        result.sort();
        result
    }
}


impl Iterator for Table {
    type Item = Row;

    fn next(&mut self) -> Option<Self::Item> {
        let mut result: Self::Item = HashMap::new();

        let mut changed = false;

        for column_name in self.columns.keys().into_iter() {
            let value = match self.columns.get(column_name).expect("Column Name doesn't exist").get(self.current_index) {
                None => "",
                Some(val) => {
                    changed = true;
                    &val.clone()
                }
            };

            result.insert(column_name.clone(), value.parse().unwrap());
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

#[cfg(test)]
mod tests {
    use super::*;

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


    #[test]
    fn test_table() {
        let table: Table = Table::from_json_array(get_test_json_table());

        let column = table.get_column_names().get(0).unwrap().clone();

        println!("{:?}", &table.get_column_names());
        println!("{:?}", &column);

        for row in table.into_iter() {
            println!("{}", row.get(column.as_str()).unwrap());
            // println!("{:?}", row);
        }
    }
}
