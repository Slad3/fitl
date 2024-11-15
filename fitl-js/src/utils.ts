export function cleanMap(table: Map<string, any>[]): Record<string, any>[] {
    return table.map((map) => Object.fromEntries(map));
}

export function mapToObj(map: Map<string, any>): Record<string, any> {
    const obj = Object.fromEntries(
        [...map].map(([key, value]) => [key, value instanceof Map ? mapToObj(value) : value])
    );
    return obj;
}