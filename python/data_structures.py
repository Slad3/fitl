from dataclasses import dataclass
from enum import Enum
from pprint import pprint


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


@dataclass
class Operation:
    column: str
    operation: ComparisonOperator
    value: str
    negated: bool = False
    case_sensitive: bool = False

    def __str__(self) -> str:
        return f"{self.column}{self.operation}|{self.value}"


NEGATE_VALUE = "!"

COMPARISON_OPERATORS_NAMES = [operation.name for operation in ComparisonOperator]
COMPARISON_OPERATORS = [operation.value for operation in ComparisonOperator]
BOOLEAN_COMPARISON_OPERATORS_NAMES = [operation.name for operation in BooleanComparisonOperator]
BOOLEAN_COMPARISON_OPERATORS = [operation.value for operation in BooleanComparisonOperator]

Operation_Stack = type[Operation | BooleanComparisonOperator]


def if_boolean_comparison_operator(input_str: str):
    if input_str in BOOLEAN_COMPARISON_OPERATORS:
        return BooleanComparisonOperator(input_str)
    elif input_str.upper() in BOOLEAN_COMPARISON_OPERATORS_NAMES:
        return BooleanComparisonOperator[input_str.upper()]
    else:
        return None


def if_comparison_operator(input_str: str):
    if input_str in COMPARISON_OPERATORS:
        return ComparisonOperator(input_str)
    elif input_str.upper() in COMPARISON_OPERATORS_NAMES:
        return ComparisonOperator[input_str.upper()]
    else:
        return None


def tokenize(input_string: str) -> list[str] | str:
    split_string = input_string.split()

    result_list = []
    start_index = -1
    in_quotes = False

    enumerated_split_string = enumerate(split_string)
    for index, _ in enumerated_split_string:
        value = split_string[index]

        if index > 15:
            print(index, value)

        if split_string[index][0] == "(" and not in_quotes:
            result_list.append("(")
            value = value[1:]

        if split_string[index][0:2] == "!(" and not in_quotes:
            result_list.append("!")
            result_list.append("(")
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


def test_tokenize():
    input_query = r'artist := Pac & album =: "Against The World"'
    tokens = tokenize(input_query)
    assert tokens == ['artist', ':=', 'Pac', '&', 'album', '=:', 'Against The World']


def test_tokenize_with_parentheses():
    input_query = r'artist := Pac & (album =: "Against The World" | album =: "Strictly 4")'
    tokens = tokenize(input_query)
    assert tokens == ['artist', ':=', 'Pac', '&', '(', 'album', '=:', 'Against The World', '|', 'album', '=:',
                      'Strictly 4', ')']


def test_tokenise_with_negated_parentheses():
    input_query = r'artist := Pac & !(album =: "Against The World" | album =: "Strictly 4")'
    tokens = tokenize(input_query)
    assert tokens == ['artist', ':=', 'Pac', '&', '!', '(', 'album', '=:', 'Against The World', '|', 'album', '=:',
                      'Strictly 4', ')']

def test_tokenise_with_negated_nested_parentheses():
    print()
    input_query = r'artist =: Pac & (album =: "Against The World" | (album =: "Strictly 4" & title = "I get Around"))'
    tokens = tokenize(input_query)
    print(tokens)
    assert tokens == ['artist', ':=', 'Pac', '&', '!', '(', 'album', '=:', 'Against The World', '|', '(', 'album', '=:',
                      'Strictly 4', '&', 'title', '=', 'I get Around', ')', ')']
