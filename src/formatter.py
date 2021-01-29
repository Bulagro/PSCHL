from collections import namedtuple
from tokenizer import *


Line = namedtuple('Line', 'indent tokens')

def get_lines(tokens_list: list):
    indent_level = 0
    increase_indent = False
    line_tokens = []
    formatted_lines = []

    for token in tokens_list:
        if token.type == Type.NewLine:
            formatted_lines.append(Line(indent_level, line_tokens))
            line_tokens = []

            if increase_indent:
                indent_level += 1
                increase_indent = False

        elif token.type == Type.OKeyword:
            increase_indent = True
            line_tokens.append(token)

        elif token.type == Type.CKeyword:
            if indent_level > 0:
                indent_level -= 1

            line_tokens.append(token)
            increase_indent = False

        else:
            line_tokens.append(token)

    if line_tokens:
        formatted_lines.append(Line(indent_level, line_tokens))

    return formatted_lines
