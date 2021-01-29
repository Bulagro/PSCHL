import unittest, json
from formatter import *
from tokenizer import *

with open('config/es.json', 'r') as f:
    keywords = json.load(f)['keywords']

class FormatterTest(unittest.TestCase):
    def test_empty_input(self):
        tokens = []
        lines = get_lines(tokens)

        self.assertEqual([], lines)

    def test_single_new_line_adds_empty_line(self):
        tokens = tokenize('\n', keywords)
        lines = get_lines(tokens)

        self.assertEqual(
            [Line(0, [])],
            lines
        )

    def test_single_line(self):
        tokens = tokenize('single line 123 -', keywords)
        lines = get_lines(tokens)

        self.assertEqual(
            [Line(0, [
                Token(Type.Identifier, 'single'),
                Token(Type.Identifier, 'line'),
                Token(Type.Number, '123'),
                Token(Type.Operator, '-'),
            ])],
            lines
        )

    def test_lines_with_same_indent_level(self):
        tokens = tokenize('single line 123 - \n this is another line', keywords)
        lines = get_lines(tokens)

        self.assertEqual(
            [
                Line(0, [
                    Token(Type.Identifier, 'single'),
                    Token(Type.Identifier, 'line'),
                    Token(Type.Number, '123'),
                    Token(Type.Operator, '-'),
                ]),
                Line(0, [
                    Token(Type.Identifier, 'this'),
                    Token(Type.Identifier, 'is'),
                    Token(Type.Identifier, 'another'),
                    Token(Type.Identifier, 'line'),
                ]),
            ],
            lines
        )

    def test_opening_keyword_increases_indent_level_for_the_next_line(self):
        tokens = tokenize('''si 1 + 0 entonces
        haz algo''', keywords)
        lines = get_lines(tokens)

        self.assertEqual(
            [
                Line(0, [
                    Token(Type.OKeyword, 'si'),
                    Token(Type.Number, '1'),
                    Token(Type.Operator, '+'),
                    Token(Type.Number, '0'),
                    Token(Type.Keyword, 'entonces'),
                ]),
                Line(1, [
                    Token(Type.Identifier, 'haz'),
                    Token(Type.Identifier, 'algo'),
                ]),
            ],
            lines
        )

    def tested_nested_opening_keywords(self):
        tokens = tokenize(''' si a entonces
        si b entonces
        1
        ''', keywords)
        lines = get_lines(tokens)

        self.assertEqual(
            [
                Line(0, [
                    Token(Type.OKeyword, 'si'),
                    Token(Type.Identifier, 'a'),
                    Token(Type.Keyword, 'entonces'),
                ]),
                Line(1, [
                    Token(Type.OKeyword, 'si'),
                    Token(Type.Identifier, 'b'),
                    Token(Type.Keyword, 'entonces'),
                ]),
                Line(2, [
                    Token(Type.Number, '1'),
                ]),
            ],
            lines
        )

    def test_oudentation(self):
        tokens = tokenize('''si a entonces b
        finsi''', keywords)
        lines = get_lines(tokens)

        self.assertEqual(
            [
                Line(0, [
                    Token(Type.OKeyword, 'si'),
                    Token(Type.Identifier, 'a'),
                    Token(Type.Keyword, 'entonces'),
                    Token(Type.Identifier, 'b'),
                ]),
                Line(0, [
                    Token(Type.CKeyword, 'finsi')
                ]),
            ],
            lines
        )

    def test_outdent_cant_be_below_0(self):
        tokens = tokenize('finsi', keywords)
        lines = get_lines(tokens)

        self.assertEqual(
            [Line(0, [Token(Type.CKeyword, 'finsi')])],
            lines
        )


if __name__ == "__main__":
    unittest.main()
