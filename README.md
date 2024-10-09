# FiTL (Filter Table (Query) Language)

### Pronounced like "Fiddle"

A simple easy to learn and remember language used to filter down a table of data within a simple string. Easy to implement, easy to write, but also easy to specify and detail.

## How does it differ from SQL?
- Designed specifically to write quickly in the moment
- Filters single tables only (no joining tables or referencing other tables)
- Super easy for non-programmer people to learn, though deep enough to master and create complex and detailed queries
- Easy to quickly write within a text box, even on mobile (looking at you regex)

## Real World Examples of Where This Would Be Implemented

- Spotify Playlist/Liked Songs Search Box
    - Query example: ```artist = Outkast & album != Idlewild```
- Product search pages
    - Query example ```brand != Apple & (ram = 32GB | resolution = "2560x1440")```

## Writing queries

### For programmers: writing queries is similar to writing boolean conditionals

### For people who touch grass:

The simplest query is based on the base operation schema:

```
<column name> <comparison operation> <compared to value>  
```

For example

```
artist = Prince
```

From a data table (for example a song playlist), will only return the rows where the `artist`(<column>) is equal to
`Prince`

This can be expanded upon with a boolean operator to combine operations

```
artist = Prince & title = "When Doves Cry"
```

You can inverse this filter by an exclamation point (parentheses are recommended, but not required)

```
!(artist = Prince & title = "When Doves Cry")
```

Like in PEMDAS, Operations are executed from left to right. Parentheses, like in math, will prioritize operations in a
specified order.

```
album = 1999 | (artist = Prince & title = "When Doves Cry")
```

Also on the topic of word odering: no, Yodaing is not allowed

```
Prince = artist // Will not work
```

A somewhat extreme example:

```
artist ^= 2Pac | artist =: pac | artist = Makaveli & (album =: theory or title =: "Ain't Hard 2 Find")
```

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
- Add more ways to create a table
- Add build scripts for
    - Standard Rust library
    - WASM js library
    - Compiled python library
- Build and deploy simple frontend for easy public testing 
