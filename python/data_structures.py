from dataclasses import dataclass
from enum import Enum


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

    def __str__(self):
        return str(self.value)


@dataclass
class Operation:
    column: str
    operation: ComparisonOperator
    value: str
    negated: bool = False
    case_sensitive: bool = False

    def __str__(self) -> str:
        return f"{self.column} | {self.operation} | {self.value}"


NEGATE_VALUE = "!"
CASE_SENS_VALUE = "^"

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

    for index, value in enumerate(split_string):

        if value.startswith('"'):
            start_index = index
            in_quotes = True

        if not in_quotes:
            while value and value[0] in [NEGATE_VALUE, CASE_SENS_VALUE, "("]:
                if value[0] == "(":
                    result_list.append("(")
                elif value[0] == CASE_SENS_VALUE:
                    result_list.append(CASE_SENS_VALUE)
                elif value[0] == NEGATE_VALUE:
                    result_list.append(NEGATE_VALUE)

                value = value[1:]

        reversed_stack = []
        temp_reversed = value[::-1]
        while temp_reversed[0] == ")":
            reversed_stack.append(")")
            temp_reversed = temp_reversed[1:]

        value = temp_reversed[::-1]

        if in_quotes and value.endswith('"'):
            temp_token = " ".join(split_string[start_index:index] + [value])[1:-1]  # Join and remove quotes
            result_list.append(temp_token)
            in_quotes = False
        elif not in_quotes:
            result_list.append(value)

        if len(reversed_stack) > 0 and not in_quotes:
            result_list.extend(reversed_stack[::-1])

    return result_list


def test_tokenize():
    print()
    input_query = r'artist := Pac & album =: "Against The World"'
    tokens = tokenize(input_query)
    assert tokens == ['artist', ':=', 'Pac', '&', 'album', '=:', 'Against The World']


def test_tokenize_with_parentheses():
    print()
    input_query = r'artist := Pac & (album =: "Against The World" | album =: "Strictly 4")'
    tokens = tokenize(input_query)
    assert tokens == ['artist', ':=', 'Pac', '&', '(', 'album', '=:', 'Against The World', '|', 'album', '=:',
                      'Strictly 4', ')']


def test_tokenize_with_negated_parentheses():
    print()
    input_query = r'artist := Pac & !(album =: "Against The World" | album =: "Strictly 4")'
    tokens = tokenize(input_query)
    assert tokens == ['artist', ':=', 'Pac', '&', '!', '(', 'album', '=:', 'Against The World', '|', 'album', '=:',
                      'Strictly 4', ')']


def test_tokenize_with_negated_nested_parentheses():
    print()
    input_query = r'artist =: Pac & !(album =: "Against The World" | (album =: "Strictly 4" & title = "I get Around"))'
    tokens = tokenize(input_query)
    assert tokens == ['artist', '=:', 'Pac', '&', '!', '(', 'album', '=:', 'Against The World', '|', '(', 'album', '=:',
                      'Strictly 4', '&', 'title', '=', 'I get Around', ')', ')']


def test_tokenize_multiple_negates():
    print()
    input_query = r'!!!!!!((artist = Pac) & (title !^= "Big Poppa"))'
    tokens = tokenize(input_query)
    assert tokens == ['!', '!', '!', '!', '!', '!', '(', '(', 'artist', '=', 'Pac', ')', '&', '(', 'title', '!', '^',
                      '=',
                      'Big Poppa', ')', ')']
