from pprint import pprint

import pytest

from data_structures import Operation, BooleanComparisonOperator, if_boolean_comparison_operator, \
    if_comparison_operator, NEGATE_VALUE, ComparisonOperator, tokenize, COMPARISON_OPERATORS, \
    COMPARISON_OPERATORS_NAMES, Operation_Stack


def parse(tokens: list[str]) -> list[Operation_Stack] | str:
    """
        TODO:
        - Support more than one level of parentheses
    """
    op_stack = []

    def parse_S(temp_tokens: list[str], temp_stack=None) -> (
            list[Operation | BooleanComparisonOperator], list[str]):
        if temp_stack is None:
            nonlocal op_stack
        else:
            op_stack = temp_stack

        if temp_tokens[0] == "!":
            temp_tokens = parse_Neg(temp_tokens[1:], op_stack)
        elif temp_tokens[0] == "(":
            temp_tokens = parse_Par(temp_tokens[1:], op_stack)
        else:
            temp_tokens = parse_Op(temp_tokens, op_stack)
        if len(temp_tokens) > 0:
            bool_op_result = if_boolean_comparison_operator(temp_tokens[0])
            if bool_op_result is not None:
                op_stack.append(bool_op_result)
                temp_tokens = temp_tokens[1:]
                temp_tokens = parse_S(temp_tokens, op_stack)

        return temp_tokens

    def parse_Op(temp_tokens: list[str], temp_stack=None) -> list[str]:
        if temp_stack is None:
            nonlocal op_stack
        else:
            op_stack = temp_stack

        if len(temp_tokens) >= 3:
            negated = False
            case_sensitive = False

            compare_op = temp_tokens[1]
            if compare_op[0] == "!":
                negated = True
                compare_op = compare_op[1:]

            if compare_op[0] == "^":
                case_sensitive = True
                compare_op = compare_op[1:]

            operation = if_comparison_operator(compare_op)

            if operation is None:
                raise f"Operation {temp_tokens[1]} not recognized: {temp_tokens}"

            column = temp_tokens[0]
            value = temp_tokens[2]

            op_stack.append(Operation(
                column=column,
                operation=operation,
                value=value,
                negated=negated,
                case_sensitive=case_sensitive,
            ))

            return temp_tokens[3:]
        else:
            raise f"Not enough tokens for operation: {temp_tokens}"

    def parse_Par(temp_tokens: list[str], temp_stack=None) -> list[str]:
        if temp_stack is None:
            nonlocal op_stack
        else:
            op_stack = temp_stack

        tempor_stack = []
        temp_tokens = parse_S(temp_tokens, tempor_stack)
        if temp_tokens[0] != ")":
            raise "No matching closing parenthesis"

        # pprint(temp_stack)
        if len(tempor_stack) == 0:
            op_stack.extend(tempor_stack)
        else:
            pprint(temp_stack)
            print()
            pprint(tempor_stack)
            print("here")
            if temp_stack[-1] == NEGATE_VALUE:
                temp_ops = temp_stack[-2:]
                print(temp_ops)
                temp_ops.reverse()
                print(temp_ops)
                tempor_stack + temp_ops + temp_stack[:-2]
            else:
                op_stack = tempor_stack + [temp_stack[-1]] + temp_stack[:-1]

        return temp_tokens[1:]

    def parse_Neg(temp_tokens: list[str], temp_stack=None) -> list[str]:
        if temp_stack is None:
            nonlocal op_stack
        else:
            op_stack = temp_stack

        op_stack.extend([NEGATE_VALUE])
        temp_tokens = parse_S(temp_tokens[1:])
        return temp_tokens

    temp_tokens = parse_S(tokens)
    if len(temp_tokens) != 0:
        pprint(temp_tokens)
        raise f"Ya screwed up"

    return op_stack



def test_compile_simple_success():
    print()
    input_query = f"artist =: Pac"

    pprint(tokenize(input_query))

    operation_stack = parse(tokenize(input_query))
    print(operation_stack)
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
        Operation(column='album', operation=ComparisonOperator.CONTAINS, value='Against The World'),
        BooleanComparisonOperator.OR,
        Operation(column='album', operation=ComparisonOperator.CONTAINS, value='Strictly 4'),
        BooleanComparisonOperator.AND,
        Operation(column='artist', operation=ComparisonOperator.CONTAINS, value='Pac')]


def test_compile_nested_parentheses_success():
    print()
    tokens = ['artist', ':=', 'Pac', '&', '!', '(', 'album', '=:', 'Against The World', '|', '(', 'album', '=:',
              'Strictly 4', '&', 'title', '=', 'I get Around', ')', ')']
    result = parse(tokens)
    # pprint(result)
    assert result == [
        Operation(column='album', operation=ComparisonOperator.CONTAINS, value='Strictly 4'),
        BooleanComparisonOperator.AND,
        Operation(column='title', operation=ComparisonOperator.EQUALS, value='I get Around'),
        BooleanComparisonOperator.OR,
        Operation(column='album', operation=ComparisonOperator.CONTAINS, value='Against The World'),
        '!',
        BooleanComparisonOperator.AND,
        Operation(column='artist', operation=ComparisonOperator.CONTAINS, value='Pac')]
