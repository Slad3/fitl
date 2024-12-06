import { describe, it, expect } from 'vitest';
import { fitlFilter, Options } from './index'

const exampleTable = [
    { name: "apple", category: "fruit", color: "red", amount: 3 },
    { name: "banana", category: "fruit", color: "yellow", amount: 4 },
    { name: "beans", category: "legumes", color: "red", amount: 5 },
    { name: "steak", category: "meat", color: "red", amount: 82 },
    { name: "steak cutlets", category: "meat", color: "red", amount: 4 },
    { name: "pork rinds", category: "seasoned air", color: "brown", amount: 1 },
    { name: "pork", category: "meat", color: "red", amount: 9 },
    { name: "pork cutlets", category: "meat", color: "red", amount: 54 },
    { name: "pork belly", category: "meat", color: "red", amount: 13 },
    { name: "fries", category: "carb", color: "brown", amount: 72 },
    { name: "steak fries", category: "carb", color: "brown", amount: 24 },
    { name: "squash", category: "vegetable", color: "yellow", amount: 15 },
    { name: "bread", category: "carb", color: "brown", amount: 3.34 },
]

describe('fitlFilter tests no options', () => {
    it('simple success', async () => {
        const queryInput = "name =: apple & color = red"
        let result = await fitlFilter(queryInput, exampleTable)
        expect(result).toEqual([{ name: "apple", category: "fruit", color: "red", amount: "3" }])
    });

    it('simple success options', async () => {
        const queryInput = "name =: apple & color = red"
        const options: Options = {
            tableFormat: "JSARRAY",
            columnTypes: {
                amount: "number"
            }
        }
        let result = await fitlFilter(queryInput, exampleTable, options)
        expect(result).toEqual([{ name: "apple", category: "fruit", color: "red", amount: 3 }])
    });

    it('options invalid column', async () => {
        const queryInput = "name =: apple & color = red"
        const options: Options = {
            tableFormat: "JSARRAY",
            columnTypes: {
                amount: "number",
                asdf: "number"
            }
        }

        try {
            await fitlFilter(queryInput, exampleTable, options)
        } catch (e) {
            expect(e).toEqual({
                error: {
                    details: 'ColumnParsingError(ColumnNotFound("asdf"))',
                    level: 'TableError'
                }
            })

        }
    });
});