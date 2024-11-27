// import init, { fitl_filter_custom_table_format } from 'fitl-wasm'
import init, { fitl_filter_custom_table_format } from '../fitl-wasm/pkg/'
import { buffer } from "src/fitl-wasm.wasm"
import { cleanMap, mapToObj } from './utils'

let wasmInitialized = false;

/**
 * @typedef {Object} Options
 * @property {string} tableFormat Specific format for inputted table. Default "JSARRAY"
 */
export type Options = {
    tableFormat?: string
}

const defaultOptions: Options = {
    tableFormat: "JSARRAY"
}


function replaceObjectValues(inputOptions: Options, defaultOptions: Options): Options {
    const result: Options = { ...defaultOptions };

    for (const key in inputOptions) {
        if (key in defaultOptions) {
            result[key as keyof Options] = inputOptions[key as keyof Options];
        }
    }

    return result;
}

function parseOptions(inputOptions?: Options) {
    if (inputOptions === undefined) return defaultOptions

    return replaceObjectValues(inputOptions, defaultOptions)
}


/**
 * 
 * @param query Query String
 * @param input_table Table to be inputted
 * @param options Optional options for both table format and query options 
 * @returns New table filtered in the same format as inputed original table
 */
export async function fitlFilter(query: string, input_table: any, options?: Options) {
    if (!wasmInitialized) await init(buffer)

    const inputOptions = parseOptions(options);

    try {
        let result = fitl_filter_custom_table_format(query, input_table, inputOptions.tableFormat!)

        if (inputOptions.tableFormat === "JSARRAY") {
            return cleanMap(result)
        } else {
            return result
        }
    } catch (error: any) {
        throw mapToObj(error);
    }

}
