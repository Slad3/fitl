from pprint import pprint
import pytest


from data_structures import BooleanComparisonOperator, \
     NEGATE_VALUE, ComparisonOperator, tokenize, Operation
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


def filter_row(operation_stack: list[Operation | BooleanComparisonOperator | str | list], row: dict) -> bool | str:
    current_bool_value: bool = True
    boolean_comparison_operator: BooleanComparisonOperator = None

    for index, operation in enumerate(operation_stack):
        if operation == NEGATE_VALUE:
            current_bool_value = not current_bool_value

        if type(operation) is BooleanComparisonOperator:
            boolean_comparison_operator = operation

        if str(type(operation)) == "<class 'python.data_structures.Operation'>":
            if operation.column not in row.keys():
                return f"Invalid Column {operation.column}"
            operation_result = operate(operation, row)
            boolean_comparison_operator = BooleanComparisonOperator.AND if boolean_comparison_operator is None else boolean_comparison_operator

            current_bool_value = boolean_operate(current_bool_value, operation_result, boolean_comparison_operator)

        if type(operation) is list:
            operation_result = filter_row(operation, row)
            boolean_comparison_operator = BooleanComparisonOperator.AND if boolean_comparison_operator is None else boolean_comparison_operator

            current_bool_value = boolean_operate(current_bool_value, operation_result, boolean_comparison_operator)

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


def test_filter_row(test_row):
    print()
    input_query = r'artist =: Pac & album =: "Against the World"'
    operation_stack = parse(tokenize(input_query))

    result = filter_row(operation_stack, test_row)
    assert result is True


def test_filter_case_sensitive_success(test_row):
    print()
    input_query = r'artist ^=: Pac & album =: "Against the World"'
    operation_stack = parse(tokenize(input_query))

    result = filter_row(operation_stack, test_row)

    assert result is True


def test_filter_negate_parentheses(test_row):
    print()
    input_query = r'!(artist =: Pac)'
    operation_stack = parse(tokenize(input_query))

    result = filter_row(operation_stack, test_row)

    assert result is False


def test_filter_case_sensitive_fail(test_row):
    print()
    input_query = r'artist ^= pac'
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


def test_general_testing(test_table):
    print()
    input_query = r'(artist =: Pac) | title = "Toss It Up"'
    operation_stack = parse(tokenize(input_query))

    result = filter_table(operation_stack, test_table)

    pprint(result)
