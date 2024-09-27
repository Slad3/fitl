from dataclasses import dataclass
from enum import Enum
from pprint import pprint

import pytest
import responses


class ComparisonOperator(Enum):
    EQUALS = "="
    CONTAINS = "=:"
    ISIN = ":="
    LESS_THAN = "<"
    MORE_THAN = ">"
    LESS_THAN_EQUALS = "<="
    MORE_THAN_EQUALS = ">="


class BooleanComparisonOperator(Enum):
    OR = "|"
    AND = "&"


NEGATE_VALUE = "!"


@dataclass
class Operation:
    column: str
    operation: ComparisonOperator
    value: str
    negated: bool = False
    case_sensitive: bool = False


COMPARISON_OPERATORS_NAMES = [operation.name for operation in ComparisonOperator]
COMPARISON_OPERATORS = [operation.value for operation in ComparisonOperator]
BOOLEAN_COMPARISON_OPERATORS_NAMES = [operation.name for operation in BooleanComparisonOperator]
BOOLEAN_COMPARISON_OPERATORS = [operation.value for operation in BooleanComparisonOperator]


def tokenize(input_string: str) -> list[str] | str:
    split_string = input_string.split()

    result_list = []
    start_index = -1
    in_quotes = False

    enumerated_split_string = enumerate(split_string)
    for index, _ in enumerated_split_string:
        value = split_string[index]

        if split_string[index][0] == "(" and not in_quotes:
            result_list.append("(")
            value = value[1:]

        if split_string[index][0:2] == "!(" and not in_quotes:
            result_list.append("!(")
            value = value[2:]

        if value[0] == '"':
            start_index = index
            in_quotes = True

        if not in_quotes and value[-1] == ")":
            result_list.append(value[:-1])
            result_list.append(")")
            continue

        result_list.append(value) if not in_quotes else None

        if in_quotes and value[-1] == '"':
            temp_token = " ".join(split_string[start_index:index + 1])[1:-1]
            result_list.append(temp_token)
            in_quotes = False
        elif value[-2:] == '")':
            temp_token = " ".join(split_string[start_index:index + 1])[1:-2]
            result_list.append(temp_token)
            result_list.append(")")
            in_quotes = False

    return result_list


def parse(tokens: list[str]) -> list[Operation | BooleanComparisonOperator] | str:
    """
        TODO:
        - Support more than one level of parentheses
    """
    operation_stack: list[Operation | BooleanComparisonOperator] = []
    enumerated_tokens = enumerate(tokens)

    for index, _ in enumerated_tokens:
        if tokens[index] == "(" or tokens[index] == "!(":
            right_parenth = tokens[index:].index(")")
            if tokens[index] == "(":
                ops = parse(tokens[index + 1: index + right_parenth])
                operation_stack = ops + [operation_stack[-1]] + operation_stack[:-1] if len(
                    operation_stack) > 0 else ops

            elif tokens[index] == "!(":
                ops = parse(tokens[index + 1: index + right_parenth])
                operation_stack = ops + [NEGATE_VALUE] + [operation_stack[-1]] + operation_stack[:-1] if len(
                    operation_stack) > 0 else ops + [NEGATE_VALUE]

            [next(enumerated_tokens, None) for _ in range(right_parenth)]

        elif tokens[index] in BOOLEAN_COMPARISON_OPERATORS:
            operation_stack.append(BooleanComparisonOperator(tokens[index]))

        elif tokens[index].upper() in BOOLEAN_COMPARISON_OPERATORS_NAMES:
            operation_stack.append(BooleanComparisonOperator(tokens[index].upper()))

        else:
            column: str
            operation: ComparisonOperator
            value: str
            negated: bool = False
            case_sensitive: bool = False

            column = tokens[index]
            operator_string = tokens[index + 1]

            if operator_string[0] == "!":
                negated = True
                operator_string = operator_string[1:]

            if operator_string[0] == "^":
                case_sensitive = True
                operator_string = operator_string[1:]

            if operator_string in COMPARISON_OPERATORS:
                operation = ComparisonOperator(operator_string)
            elif operator_string.upper() in COMPARISON_OPERATORS_NAMES:
                operation = ComparisonOperator[operator_string.upper()]
            else:
                return f"Operation {operator_string} not supported"

            operation_stack.append(Operation(
                column=column,
                operation=operation,
                value=tokens[index + 2],
                negated=negated,
                case_sensitive=case_sensitive,
            ))

            [next(enumerated_tokens, None) for _ in range(2)]

    return operation_stack


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


def test_tokenize():
    input_query = r'artist := Pac & album =: "Against The World"'
    tokens = tokenize(input_query)
    assert tokens == ['artist', ':=', 'Pac', '&', 'album', '=:', 'Against The World']


def test_tokenize_with_parentheses():
    input_query = r'artist := Pac & (album =: "Against The World" | album =: "Strictly 4")'
    tokens = tokenize(input_query)
    assert tokens == ['artist', ':=', 'Pac', '&', '(', 'album', '=:', 'Against The World', '|', 'album', '=:',
                      'Strictly 4', ')']


def test_compile_simple_success():
    input_query = f"artist =: Pac"

    operation_stack = parse(tokenize(input_query))
    assert operation_stack == [Operation(column='artist', operation=ComparisonOperator.CONTAINS, value='Pac')]


def test_compile_negated_case_sensitive_operation():
    input_query = f"artist !^= Pac"

    operation_stack = parse(tokenize(input_query))
    assert operation_stack == [
        Operation(column='artist', operation=ComparisonOperator.EQUALS, value='Pac', negated=True, case_sensitive=True)]


def test_compile_simple_success_worded():
    input_query = f"artist contains Pac"

    operation_stack = parse(tokenize(input_query))
    assert operation_stack == [Operation(column='artist', operation=ComparisonOperator.CONTAINS, value='Pac')]


def test_compile_parentheses_success():
    input_query = r'artist =: Pac & (album =: "Against The World" | album =: "Strictly 4")'

    assert parse(tokenize(input_query)) == [
        Operation(column='album', operation=ComparisonOperator.CONTAINS, value='Against The World'),
        BooleanComparisonOperator.OR,
        Operation(column='album', operation=ComparisonOperator.CONTAINS, value='Strictly 4'),
        BooleanComparisonOperator.AND,
        Operation(column='artist', operation=ComparisonOperator.CONTAINS, value='Pac')]


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
