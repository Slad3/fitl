from pprint import pprint

from data_structures import BooleanComparisonOperator, if_boolean_comparison_operator, \
    if_comparison_operator, NEGATE_VALUE, ComparisonOperator, tokenize, \
    Operation_Stack
from Python.data_structures import CASE_SENS_VALUE, Operation


def parse(tokens: list[str]) -> list[Operation_Stack] | str:

    def parse_S(temp_tokens: list[str], temp_stack=None) -> (
            list[Operation | BooleanComparisonOperator], list[str]):

        temp_stack = []

        if temp_tokens[0] == NEGATE_VALUE:
            temp_tokens, result_stack = parse_Neg(temp_tokens[1:])
            temp_stack.extend(result_stack)
        elif temp_tokens[0] == "(":
            temp_tokens, result_stack = parse_Par(temp_tokens[1:])
            temp_stack.append(result_stack)
        else:
            temp_tokens, result_operation = parse_Op(temp_tokens)
            temp_stack.append(result_operation)
        if len(temp_tokens) > 0:
            bool_op_result = if_boolean_comparison_operator(temp_tokens[0])
            if bool_op_result is not None:
                temp_stack.append(bool_op_result)
                temp_tokens, result_stack = parse_S(temp_tokens[1:])
                temp_stack.extend(result_stack)

        return temp_tokens, temp_stack

    def parse_Op(temp_tokens: list[str]) -> tuple[list[str], Operation]:
        if len(temp_tokens) >= 3:
            negated = False
            case_sensitive = False

            compare_op_index = 1

            if temp_tokens[compare_op_index] == NEGATE_VALUE:
                negated = True
                compare_op_index += 1

            if temp_tokens[compare_op_index] == CASE_SENS_VALUE:
                case_sensitive = True
                compare_op_index += 1

            compare_op = temp_tokens[compare_op_index]

            operation = if_comparison_operator(compare_op)

            if operation is None:
                raise f"Operation {temp_tokens[1]} not recognized: {temp_tokens}"

            column = temp_tokens[0]
            value = temp_tokens[1 + compare_op_index]

            return (temp_tokens[2 + compare_op_index:], Operation(
                column=column,
                operation=operation,
                value=value,
                negated=negated,
                case_sensitive=case_sensitive,
            ))
        else:
            print(temp_tokens)
            raise f"Not enough tokens for operation: {temp_tokens}"

    def parse_Par(temp_tokens: list[str]) -> (list[str], list[Operation | str | BooleanComparisonOperator | list]):

        par_stack = []
        temp_tokens, result_stack = parse_S(temp_tokens)
        par_stack.extend(result_stack)
        while temp_tokens[0] != ")":
            temp_tokens, result_stack = parse_S(temp_tokens)
            par_stack.extend(result_stack)

        temp_tokens = temp_tokens[1:]
        return temp_tokens, par_stack

    def parse_Neg(temp_tokens: list[str]) -> (list[str], list[Operation | str | BooleanComparisonOperator | list]):
        temp_stack = []
        temp_stack.extend([NEGATE_VALUE])
        temp_tokens, result_stack = parse_S(temp_tokens)
        temp_stack.extend(result_stack)
        return temp_tokens, temp_stack

    temp_tokens, operation_stack = parse_S(tokens)
    if len(temp_tokens) != 0:
        pprint(temp_tokens)
        raise f"Ya screwed up"

    return operation_stack


def test_compile_simple_success():
    print()
    input_query = f"artist =: Pac"

    operation_stack = parse(tokenize(input_query))
    assert operation_stack == [Operation(column='artist', operation=ComparisonOperator.CONTAINS, value='Pac')]


def test_compile_negated_case_sensitive_operation():
    print()
    input_query = f"artist !^= Pac"

    operation_stack = parse(tokenize(input_query))
    assert operation_stack == [
        Operation(column='artist', operation=ComparisonOperator.EQUALS, value='Pac', negated=True, case_sensitive=True)]


def test_compile_simple_success_worded():
    print()
    input_query = f"artist contains Pac"

    operation_stack = parse(tokenize(input_query))
    assert operation_stack == [Operation(column='artist', operation=ComparisonOperator.CONTAINS, value='Pac')]


def test_compile_parentheses_success():
    print()
    input_query = r'artist =: Pac & (album =: "Against The World" | album =: "Strictly 4")'

    result = parse(tokenize(input_query))
    assert result == [
        Operation(column='artist', operation=ComparisonOperator.CONTAINS, value='Pac'),
        BooleanComparisonOperator.AND,
        [
            Operation(column='album', operation=ComparisonOperator.CONTAINS, value='Against The World'),
            BooleanComparisonOperator.OR,
            Operation(column='album', operation=ComparisonOperator.CONTAINS, value='Strictly 4')
        ]
    ]


def test_compile_nested_parentheses_success():
    print()
    tokens = ['artist', ':=', 'Pac', '&', '!', '(', 'album', '=:', 'Against The World', '|', '(', 'album', '=:',
              'Strictly 4', '&', 'title', '=', 'I get Around', ')', ')']
    result = parse(tokens)
    assert result == [
        Operation(column='artist', operation=ComparisonOperator.ISIN, value='Pac'),
        BooleanComparisonOperator.AND,
        NEGATE_VALUE,
        [
            Operation(column='album', operation=ComparisonOperator.CONTAINS, value='Against The World'),
            BooleanComparisonOperator.OR,
            [
                Operation(column='album', operation=ComparisonOperator.CONTAINS, value='Strictly 4'),
                BooleanComparisonOperator.AND,
                Operation(column='title', operation=ComparisonOperator.EQUALS, value='I get Around')
            ]
        ]
    ]
