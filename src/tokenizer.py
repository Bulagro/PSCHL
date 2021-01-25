import string, json
from enum import Enum


class Type(Enum):
    Keyword  = 'keyword'
    Number   = 'number'
    Operator = 'operator'
    NewLine  = 'newline'
    String   = 'string'
    Other    = 'other'


class Token:
    def __init__(self, t, c=None):
        self.type = t
        self.content = c

    def __eq__(self, other):
        if not type(self) == type(other):
            return False

        return self.type == other.type and self.content == other.content

    def __repr__(self):
        return f'{self.type}<{self.content}>'


def tokenize(str_input: str):
    IDENTIFIER_CHARS = string.ascii_letters + '_'
    NUM_CHARS = string.digits + '.'

    token_list = []
    token_type = None
    token_content = ''

    for i in range(len(str_input)):
        if str_input[i] in IDENTIFIER_CHARS:
            if not token_content:
                token_type = Type.Other

            if token_type == Type.Number:
                token_list.append(Token(token_type, token_content))
                token_type = Type.Other
                token_content = ''

            token_content += str_input[i]

        elif str_input[i] in NUM_CHARS:
            if not token_content:
                token_type = Type.Number

            token_content += str_input[i]

        elif str_input[i] == ' ':
            if token_content:
                token_list.append(Token(token_type, token_content))
                token_content = ''

    if token_content:
        token_list.append(Token(token_type, token_content))

    return token_list
