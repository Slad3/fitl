# FiTL (Filter Table Language)

Source and rust crate for FiTL

## Symbols Dictionary

T -> Generic Symbol (For below chart purposes only)

\<value> -> inputted value

| Word           | Symbol    | Description                                                                                                      |
|----------------|-----------|------------------------------------------------------------------------------------------------------------------|
| not            | !T        | Negates Operation                                                                                                |
| is / equals    | =         | Exact match                                                                                                      |
| contains       | =:        | Left contains right (Nickolas Picklous =: Nick)                                                                  |
| isin           | :=        | Right contains left (Nick := Nickolas Picklous)                                                                  |
| lessthan       | <         | "Less than" comparison of numbers or of characters/strings based on ASCII value of characters                    |
| morethan       | \>        | "greater than" comparison of numbers or of characters/strings based on ASCII value of characters                 |
| lessthanequals | <=        | "Less than or equals" comparison of numbers or of characters/strings based on ASCII value of characters          |
| morethan       | \>=       | "greater than or equals" comparison of numbers or of characters/strings based on ASCII value of characters       |
| or             | \|        | Or boolean operation                                                                                             |
| and            | &         | And boolean operation                                                                                            |
| *Parenthesis*  | ()        | Prioritizes statements inside parenthesis                                                                        |
| *NA*           | "<value>" | Combines multiple words into single string. Necessary for multi-worded tokens, optional for single worded tokens |
| *NA*           | ^T        | Makes statement case sensitive queries are case insensitive by default                                           |

## NFA Schema

```
S [boolean] -> Op | Par | Neg | S BoolOp S

Op [boolean] -> column ComparOp value | column CaseSens value | column NegSym value

Par [boolean] -> (S) | (S) BoolOp S

Neg [boolean] ->!S

BoolOp [sum] ->  & | |   // '&' or '|' 

value [token] -> value | "value"

ComparOp [sym] -> = | < | > | <= | >= | =: | :=

CaseSens [Sym] -> ^ComparOp 

NegSym [sym] -> !ComparOp | !BoolOp | !CaseSens

```

### TODO

- Add "re" Comparison symbol for regex matching
- Add "~" symbol for a soft match excluding any non-alphanumeric characters in matching
- Add custom type structures for table columns
- Add chaining |/& statements ('artist =: roots | "Black Thought" | "Prince"')
- Add more ways to create a table
- Add build scripts for
    - Standard Rust library
    - WASM js library
    - Compiled python library
- Build and deploy simple frontend for easy public testing
