# fitl-js

TS/JS library wrapper for fitl-wasm and fitl-rs. Mean to make implementing fitl to a javascript based page as easy as possible, while also solving clunkiness issues when it comes to packaging and compiling wasm based libraries.

For a full documentation/information about FiTL and how to write
queries: [GitHub](https://github.com/Slad3/FilterTableLanguage)

## Getting Started

### Installation

```bash
npm install fitl-js
pnpm install fitl-js
yarn install
```

### Hello World

```typescript
import  { fitlFilter } from 'fitl-js';

let tableData = [
    { category: "meat" },
    { category: "fruit" }
];

let query = "category = fruit";

async function main(){
    try {
        let resultTable = await fitlFilter(query, tableData);
    } catch (error: unknown) {
        console.error(error);
    }
}
main();
```

Options are optional of course, currently used to specify input/output table types with other future options coming soon.

```typescript

import  { fitlFilter, type Options } from 'fitl-js';

let tableData = [
    { category: "meat" },
    { category: "fruit" }
];

let query = "category = fruit";

// Default tableFormat is JSARRAY, other table formats coming soon
let options: Options = { tableFormat: 'JSARRAY' };

async function main(){
    try {
        let resultTable = await fitlFilter(query, tableData, options);
    } catch (error: unknown) {
        console.error(error);
    }
}
main();
```

## Building from Source

To build fitl-js from source

Requirements:

- Rust >= 1.8
- Nodejs >= v22

Once everything is installed, clone the repo and build local dependencies

```bash
git clone https://github.com/Slad3/FilterTableLanguage.git
cd FilterTableLanguage/fitl-rs
cargo build
cd ../fitl-js/fitl-wasm
cargo build
wasm-pack build --target web --release
cd ..
npm install
```

Next package fitl-wasm into a singular TS file so we don't have to worry about packaging, serving, and sending .wasm files

```bash
npm run wasminstall
```

When ran successfully, this command should put a `fitl-wasm.wasm.ts` file in the `fitl-js/src` folder. If it didn't run successfully, check your fitl-wasm/pkg folder for correctly built files or if wasmwrap got installed correctly

Run an `npm run build` to build the library to the `fitl-js/dist` folder, or an 'npm run test' to just run all jest tests.
