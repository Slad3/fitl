# Fitl-WASM

Wasm wrapper for FiTL, specifically the fitl-rust crate.

For a full documentation/information about FiTL and how to write
queries: [GitHub](https://github.com/Slad3/FilterTableQueryLanguage)

## Getting Started

Before all else, since it does run in wasm, you need to initialize before any other functions are ran

```javascript
import init from "fitl-wasm";

await init();
```

After wasm has been initialized you don't have to initialize it again.

After that you can simply run

```javascript
import init, {fitl_filter} from "fitl-wasm";

await init();

let initialTable = [
    {column1: "a", column2: "b"},
    {column1: "aa", column2: "bcd"}
];

let query = "column2 =: c";

try {
    let resultTable = fitl_filter(query, initialTable);
} catch (error) {
    console.error(error);
}
```

`initialTable` stays untouched when passed through `fitl_filter`, `resultTable` is a new table in memory made by the
function.

This also assumes that the structure of your table in js is similar to the one above. As long as it's an array of
objects with the same single level key names. Any invalid table input will return a table parsing error with details
about what went wrong.

If you also want to just check syntax without running the entire filter, you can simply run

```javascript
import init, {check_syntax} from "fitl-wasm";

await init();

let columns = ["column1", "column2"];

let query = "column2 =: c";

try {
    let result = check_syntax(query, columns);
} catch (error) {
    console.error(error);
}
```

Which will return a `true` if syntax compiles, or a compiler error will be thrown with details about syntax errors