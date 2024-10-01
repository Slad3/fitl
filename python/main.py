from dataclasses import dataclass
from enum import Enum
from pprint import pprint

import pytest

from data_structures import Operation, BooleanComparisonOperator, if_boolean_comparison_operator, \
    if_comparison_operator, NEGATE_VALUE, ComparisonOperator, tokenize

from parsing import parse


def operate(operation: Operation, row: dict):
    result: bool

    operation_value = operation.value
    row_value = row[operation.column]

    if not operation.case_sensitive:
        operation_value = operation_value.lower()
        row_value = row_value.lower()

    match operation.operation:
        case ComparisonOperator.EQUALS:
            result = operation_value == row_value
        case ComparisonOperator.CONTAINS:
            result = operation_value in row_value
        case ComparisonOperator.ISIN:
            result = row_value == operation_value
        case ComparisonOperator.LESS_THAN:
            result = row_value < operation_value
        case ComparisonOperator.LESS_THAN_EQUALS:
            result = row_value <= operation_value
        case ComparisonOperator.MORE_THAN:
            result = row_value > operation_value
        case ComparisonOperator.MORE_THAN_EQUALS:
            result = row_value >= operation_value
        case _:
            raise f"Operation {operation.operation} not supported"

    return result if not operation.negated else not result


def boolean_operate(a: bool, b: bool, boolean_operation: BooleanComparisonOperator):
    match boolean_operation:
        case BooleanComparisonOperator.OR:
            return a or b
        case BooleanComparisonOperator.AND:
            return a and b


def filter_row(operation_stack: list[Operation | BooleanComparisonOperator], row: dict) -> bool | str:
    current_bool_value: bool = True
    boolean_comparison_operator: BooleanComparisonOperator

    for index, operation in enumerate(operation_stack):
        if type(operation) is BooleanComparisonOperator:
            boolean_comparison_operator = operation
        if type(operation) is Operation:
            if operation.column not in row.keys():
                return f"Invalid Column {operation.column}"
            operation_result = operate(operation, row)
            if index == 0:
                current_bool_value = operation_result
            else:
                current_bool_value = boolean_operate(current_bool_value, operation_result, boolean_comparison_operator)
        if operation == "!":
            current_bool_value = not current_bool_value

    return current_bool_value


def filter_table(operation_stack: list[Operation | BooleanComparisonOperator], table: list[dict]) -> list[dict] | str:
    result = []
    for row in table:
        filter_row_result = filter_row(operation_stack, row)
        if filter_row_result is str:
            return filter_row_result
        else:
            result.append(row) if filter_row(operation_stack, row) else None

    return result


def usq_filter(query: str, table: list[dict]) -> list[dict] | str:
    tokens = tokenize(query)
    if tokens is str:
        return tokens

    operation_stack = parse(tokens)
    if operation_stack is str:
        return operation_stack

    return filter_table(operation_stack, table)


@pytest.fixture
def test_row():
    return {"artist": "2Pac", "album": "Me Against the World", "title": "So Many Tears"}


@pytest.fixture
def test_table():
    return [
        {"artist": "2Pac", "album": "Me Against the World", "title": "So Many Tears"},
        {"artist": "2Pac", "album": "Me Against the World", "title": "Lord Knows"},
        {"artist": "2Pac", "album": "All Eyez on Me", "title": "All Eyez on Me"},
        {"artist": "2Pac", "album": "All Eyez on Me", "title": "2 Of Amerikaz Most Wanted"},
        {"artist": "2Pac", "album": "All Eyez on Me", "title": "Heartz of Men"},
        {"artist": "Makaveli", "album": "The Don Killuminati: The 7 Day Theory", "title": "Toss It Up"},
        {"artist": "Makaveli", "album": "The Don Killuminati: The 7 Day Theory", "title": "Me And My Girlfriend"},
        {"artist": "Makaveli", "album": "The Don Killuminati: The 7 Day Theory", "title": "Against All Odds"},
    ]


def test_compile_simple_success():
    print()
    input_query = f"artist =: Pac"

    pprint(tokenize(input_query))

    operation_stack = parse(tokenize(input_query))
    assert operation_stack == [Operation(column='artist', operation=ComparisonOperator.CONTAINS, value='Pac')]


def test_filter_row(test_row):
    input_query = r'artist =: Pac & album =: "Against the World"'
    operation_stack = parse(tokenize(input_query))

    result = filter_row(operation_stack, test_row)
    assert result is True


def test_filter_case_sensitive_success(test_row):
    input_query = r'artist ^=: Pac & album =: "Against the World"'
    operation_stack = parse(tokenize(input_query))

    result = filter_row(operation_stack, test_row)

    assert result is True


def test_filter_negate_parentheses(test_row):
    print()
    input_query = r'!(artist =: Pac) & album =: "Against the World"'
    operation_stack = parse(tokenize(input_query))
    pprint(operation_stack)

    result = filter_row(operation_stack, test_row)

    assert result is False


def test_filter_case_sensitive_fail(test_row):
    input_query = r'artist ^= pac & album =: "Against the World"'
    operation_stack = parse(tokenize(input_query))

    result = filter_row(operation_stack, test_row)
    assert result is False


def test_filter_table_simple_query(test_table):
    print()
    input_query = r'(artist = Makaveli | album =: "Against the World")'
    operation_stack = parse(tokenize(input_query))

    result = filter_table(operation_stack, test_table)

    assert result == [{'album': 'Me Against the World', 'artist': '2Pac', 'title': 'So Many Tears'},
                      {'album': 'Me Against the World', 'artist': '2Pac', 'title': 'Lord Knows'},
                      {'album': 'The Don Killuminati: The 7 Day Theory',
                       'artist': 'Makaveli',
                       'title': 'Toss It Up'},
                      {'album': 'The Don Killuminati: The 7 Day Theory',
                       'artist': 'Makaveli',
                       'title': 'Me And My Girlfriend'},
                      {'album': 'The Don Killuminati: The 7 Day Theory',
                       'artist': 'Makaveli',
                       'title': 'Against All Odds'}]
