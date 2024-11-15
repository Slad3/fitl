import { describe, it, expect } from 'vitest';
import { mapToObj } from './utils'
import { assert } from 'chai';

describe('mapToObj', () => {
    it('single layer map', () => {
        const input: Map<string, number> = new Map([
            ["key1", 1],
            ["key2", 2],
            ["key3", 3],
        ]);
        expect(mapToObj(input)).toEqual({ key1: 1, key2: 2, key3: 3 })
    });

    it('multilayer map', () => {
        const input: Map<string, any> = new Map([
            ["outerKey1", new Map([["innerKey1", 100], ["innerKey2", 200]])],
            ["outerKey2", new Map([["innerKey3", 300], ["innerKey4", 400]])],
        ]);

        expect(mapToObj(input)).toEqual({
            outerKey1: { innerKey1: 100, innerKey2: 200 },
            outerKey2: { innerKey3: 300, innerKey4: 400 }
        })
    });
});