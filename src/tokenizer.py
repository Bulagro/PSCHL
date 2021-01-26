import string, json
from enum import Enum, auto


class Type(Enum):
    Keyword   = auto()
    Number    = auto()
    Operator  = auto()
    String    = auto()
    NewLine   = auto()
    Delimiter = auto()
    Other     = auto()


class Token:
    def __init__(self, t, c=None):
        self.type = t
        self.content = c

    def __eq__(self, other):
        if not type(self) == type(other):
            return False

        return self.type == other.type and self.content == other.content

    def __repr__(self):
        if self.type == Type.NewLine:
            return f'{self.type.name}'

        return f'{self.type}<{self.content}>'


def tokenize(str_input: str):
    IDENTIFIER_CHARS = string.ascii_letters + '_'
    NUM_CHARS = string.digits + '.'
    OPERATORS = ('+', '-', '*', '/', '>', '<', '=', '!')
    DELIMITERS = ('(', ')', '[', ']', '{', '}')

    token_list = []
    token_type = None
    token_content = ''

    input_len = len(str_input)

    for i in range(input_len):
        if str_input[i] in IDENTIFIER_CHARS:
            if not token_content:
                token_type = Type.Other

            if token_type == Type.Number:
                token_type = Type.Other

            token_content += str_input[i]

        elif str_input[i] in NUM_CHARS:
            if not token_content or token_content == '-':
                token_type = Type.Number

            token_content += str_input[i]

        elif str_input[i] == '.':
            token_content += str_input[i]

        elif str_input[i] == '"':
            if token_type == Type.String:
                token_content += '"'
                token_list.append(Token(Type.String, token_content))
                token_content = ''
                token_type = None
                continue

            if not token_content:
                token_type = Type.String

            token_content += str_input[i]

        elif str_input[i] in OPERATORS:
            if not token_content:
                token_type = Type.Operator
            else:
                if token_type != Type.Operator or len(token_content) == 2:
                    token_list.append(Token(token_type, token_content))
                    token_content = ''

            token_content += str_input[i]

        elif str_input[i] == '\n':
            token_list.append(Token(Type.NewLine))
            token_content = ''

        elif str_input[i] in DELIMITERS:
            if token_content:
                token_list.append(Token(token_type, token_content))
                token_content = ''

            token_list.append(Token(Type.Delimiter, str_input[i]))
            token_content = ''

        elif str_input[i] == ' ':
            if token_content:
                if token_type == Type.String:
                    token_content += ' '
                    continue

                elif token_type == Type.Number:
                    if len(token_list) > 0 and token_list[-1].content == '-':
                        token_list.pop(-1)
                        token_content = '-' + token_content

                token_list.append(Token(token_type, token_content))
                token_content = ''

    if token_content:
        token_list.append(Token(token_type, token_content))

    return token_list
