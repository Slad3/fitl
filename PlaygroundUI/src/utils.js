export function mapToObj(map) {
    const obj = Object.fromEntries(
        [...map].map(([key, value]) => [key, value instanceof Map ? mapToObj(value) : value])
    );
    return obj;
}

export function printError(error) {
    let temp = `${error.error.level}: ${error.error.details}`
    console.error(temp);
}