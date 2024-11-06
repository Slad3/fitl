export function cleanMap(table: Map<string, any>[]): Record<string, any>[] {
    return table.map((map) => Object.fromEntries(map));
}

export function mapToObj(map: Map<string, any>): Record<string, any> {
    const obj = Object.fromEntries(
        [...map].map(([key, value]) => [key, value instanceof Map ? mapToObj(value) : value])
    );
    return obj;
}

export function printError(error: { error: { level: string; details: string } }): void {
    console.error(`${error.error.level}: ${error.error.details}`);
}

export function capitalize(s: string | unknown[]): string {
    return String(s[0]).toUpperCase() + String(s).slice(1);
}

export function sort_columns(data: Array<{ [key: string]: unknown }>, column_order: string[]): string[] {

    if (data.length <= 0) {
        return []
    }
    if (!column_order) {
        return [];
    }

    const keys = Object.keys(data[0])

    if (!column_order) {
        return keys;
    }

    let dump = [];
    let result = [];

    for (const key of keys) {
        const index = column_order.indexOf(key);
        if (index < 0) {
            dump.push(key);
        } else {
            result[index] = key;
        }
    }

    result = result.filter(Boolean);

    dump = dump.sort();

    return result.concat(dump);
}