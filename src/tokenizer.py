import string, json
from enum import Enum, auto


class Type(Enum):
    Keyword  = auto()
    Number   = auto()
    Operator = auto()
    NewLine  = auto()
    String   = auto()
    Other    = auto()


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

        elif str_input[i] == ' ':
            if token_content:
                if token_type == Type.String:
                    token_content += ' '
                else:
                    token_list.append(Token(token_type, token_content))
                    token_content = ''

    if token_content:
        token_list.append(Token(token_type, token_content))

    return token_list
