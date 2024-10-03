from data_structures import tokenize

from parsing import parse
from filter import filter_table


def usq_filter(query: str, table: list[dict]) -> list[dict] | str:
    tokens = tokenize(query)
    if tokens is str:
        return tokens

    operation_stack = parse(tokens)
    if operation_stack is str:
        return operation_stack

    return filter_table(operation_stack, table)
