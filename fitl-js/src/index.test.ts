import { describe, it, expect } from 'vitest';
import { fitl_filter, Options } from './index'

const exampleTable = [
    { name: "apple", category: "fruit", color: "red" },
    { name: "banana", category: "fruit", color: "yellow" },
    { name: "beans", category: "legumes", color: "red" },
    { name: "steak", category: "meat", color: "red" },
    { name: "steak cutlets", category: "meat", color: "red" },
    { name: "pork rinds", category: "seasoned air", color: "brown" },
    { name: "pork", category: "meat", color: "red" },
    { name: "pork cutlets", category: "meat", color: "red" },
    { name: "pork belly", category: "meat", color: "red" },
    { name: "fries", category: "carb", color: "brown" },
    { name: "steak fries", category: "carb", color: "brown" },
    { name: "squash", category: "vegetable", color: "yellow" },
    { name: "bread", category: "carb", color: "brown" },
]

describe('fitl_filter tests no options', () => {
    it('simple success', async () => {
        const queryInput = "name =: apple & color = red"
        let result = await fitl_filter(queryInput, exampleTable)
        expect(result).toEqual([{ name: "apple", category: "fruit", color: "red" }])
    });

});