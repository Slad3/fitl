import init, { fitl_filter_custom_table_format } from 'fitl-wasm'
import { buffer } from "src/fitl-wasm.wasm"
import { cleanMap } from './utils'

let wasmInitialized = false;

export type Options = {
    tableFormat?: String
}

const defaultOptions: Options = {
    tableFormat: "JSARRAY"
}

export async function fitl_filter(query: string, input_table: any, options?: Options) {
    if (!wasmInitialized) await init(buffer)

    console.log("asdfasdfas")
    try {
        return cleanMap(fitl_filter_custom_table_format(query, input_table, "JSARRAY"))

    } catch (error: any) {
        throw cleanMap(error);
    }

}
