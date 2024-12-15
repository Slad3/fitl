# [FiTL (Filter Table Language)]("https://fitl.benbarcaskey.com/")

A simple easy to learn and remember language used to filter down a table of data within a simple query string. Easy to implement, easy to write, but also easy to specify and detail.

[![fitl gif example](/PublicSite/src/lib/images/example.gif)](https://fitl.benbarcaskey.com)

## What Is Fitl?

FiTL (Filter Table Language) is a simple query langauge allowing end users to quickly and specifically filter rows from a table. Users filter rows from tables based on columns and conditional statements for those columns. Multiple conditional statemens can be combined/chained to allow for further specificity of table results. Designed to be easy to implement for developers, easy to learn and remember for general users, but also allows room to master for power users.

## Why not just use SQL? (or a modified version of sql)

- FiTL is designed to be easier to type in a simple text box, especially on mobile devices
- SQL is easy enough to learn, but FiTL is made to be easy enough for people who touch grass
- SQL allows for table joining while FiTl is limited to singular table interacitons
- In general FiTL is meant for quickly filtering an existing table for end user use while SQL is meant for more reusable structured queries. Although neither is contrained to previously said purposes.

## [Learn More on Writing Queries](https://fitl.benbarcaskey.com/queries)

## Repo Directory

- [fitl-rs](/fitl-rs) Core language compiler and runtime written in Rust
- [fitl-js](/fitl-js/) JS/TS library wrapper around fitl-wasm and fitl-rs
  - [fitl-wasm](/fitl-js/fitl-wasm/) Web Assmebly interface for fitl. Compliles from the fitl-rs library
- [PlaygroundUI](/PlaygroundUI/) Simple Example Svelte App for UI testing, updates automatically with fitl-wasm compiles
