---
tags:
---

# Main Point

- To create a universal advanced search query language for general public to do advanced searches of data from a single
  table in a single string/single search box apart from simple searches

## How does it differ from just injecting SQL?

- Super easy for people to learn, though enough to master
- Single table querying only
- Easy to use within a text box

## Example (Prototype) Queries

Examples are geared toward filtering songs in a (Spotify) playlist

### Worded Prototype

```
(artist contains 2Pac or artist is "Makaveli" or artist ~is Makaveli) and (album contains theory or title isin "Ain't Hard 2 Find");
```

### Symbolic Prototype

```
artist := Pac | artist = Makaveli | artist ~:= 2pac & (album =: theory or title := "Ain't Hard 2 Find")
```

## Symbols Dictionary

T -> Generic Symbol (For below chart purposes only)

| Word           | Symbol    | Description                                                                                                      |
|----------------|-----------|------------------------------------------------------------------------------------------------------------------|
| not            | !T        | Negates Operation                                                                                                |
| is / equals    | =         | Exact match                                                                                                      |
| contains       | =:        | Left contains right                                                                                              |
| isin           | :=        | Right contains left                                                                                              |
| lessthan       | <         | "Less than" comparison of numbers or of charactors/strings based on ASCI value of characters                     |
| morethan       | \>        | "greater than" comparison of numbers or of charactors/strings based on ASCI value of characters                  |
| lessthanequals | <=        | "Less than or equals" comparison of numbers or of charactors/strings based on ASCI value of characters           |
| morethan       | \>=       | "greater than or equals" comparison of numbers or of charactors/strings based on ASCI value of characters        |
| or             | \|        | Or boolean operation                                                                                             |
| and            | &         | And boolean operation                                                                                            |
| *Parenthesis*  | ()        | Prioritizes statements inside parenthesis                                                                        |
| *NA*           | "*token*" | Combines multiple words into single string. Necessary for multi-worded tokens, optional for single worded tokens |
| *NA*           | ^T        | Makes statement case sensitive queries are case insensitive by default                                           |

## Real World Examples of Where This Would Be Implemented

- Spotify Playlist/Liked Songs Search Box
- Product search pages

## Schema of 

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
- add "~" symbol for a soft match excluding any non-alphanumeric characters in matching
- Convert Python prototype to Rust
    - Add build scripts for
        - Standard Rust library
        - WASM js library
        - Compiled python library
    - 