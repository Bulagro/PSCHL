import unittest
from tokenizer import *

class TokenizerTest(unittest.TestCase):
    def test_single_word(self):
        input_str = 'hello'
        tokens = tokenize(input_str)

        self.assertEqual(
            [Token(Type.Other, 'hello')],
            tokens
        )

    def test_sentence(self):
        input_str = 'This are a few words'
        tokens = tokenize(input_str)

        self.assertEqual(
            [
                Token(Type.Other, 'This'),
                Token(Type.Other, 'are'),
                Token(Type.Other, 'a'),
                Token(Type.Other, 'few'),
                Token(Type.Other, 'words'),
            ],
            tokens
        )

    def test_single_number(self):
        input_str = '234'
        tokens = tokenize(input_str)

        self.assertEqual(
            [Token(Type.Number, '234')],
            tokens
        )

    def test_numbers(self):
        input_str = '234 3 3 11'
        tokens = tokenize(input_str)

        self.assertEqual(
            [
                Token(Type.Number, '234'),
                Token(Type.Number, '3'),
                Token(Type.Number, '3'),
                Token(Type.Number, '11'),
            ],
            tokens
        )

    def test_single_identifier(self):
        input_str = 'babt2002'
        tokens = tokenize(input_str)

        self.assertEqual(
            [Token(Type.Other, 'babt2002')],
            tokens
        )

    def test_multiple_identifiers(self):
        input_str = 'thes2 are24 ident1f13rs'
        tokens = tokenize(input_str)

        self.assertEqual(
            [
                Token(Type.Other, 'thes2'),
                Token(Type.Other, 'are24'),
                Token(Type.Other, 'ident1f13rs'),
            ],
            tokens
        )

    def test_mixed_numbers_and_identifiers(self):
        input_str = 'this string should have 6 elements'
        tokens = tokenize(input_str)

        self.assertEqual(
            [
                Token(Type.Other, 'this'),
                Token(Type.Other, 'string'),
                Token(Type.Other, 'should'),
                Token(Type.Other, 'have'),
                Token(Type.Number, '6'),
                Token(Type.Other, 'elements'),
            ],
            tokens
        )

    def test_identifier_that_start_with_numbers(self):
        input_str = '123onetwothree'
        tokens = tokenize(input_str)

        self.assertEqual(
            [Token(Type.Other, '123onetwothree')],
            tokens
        )

    def test_identifiers_that_start_with_numbers(self):
        input_str = '123onetwothree 2a'
        tokens = tokenize(input_str)

        self.assertEqual(
            [
                Token(Type.Other, '123onetwothree'),
                Token(Type.Other, '2a'),
            ],
            tokens
        )

    def test_single_string(self):
        input_str = ' "this is a string" '
        tokens = tokenize(input_str)

        self.assertEqual(
            [Token(Type.String, '"this is a string"')],
            tokens
        )

    def test_multiple_strings(self):
        input_str = '"a" "b" "c"'
        tokens = tokenize(input_str)

        self.assertEqual(
            [
                Token(Type.String, '"a"'),
                Token(Type.String, '"b"'),
                Token(Type.String, '"c"'),
            ],
            tokens
        )

    def test_strings_mixed_with_other_types(self):
        input_str = 'identifier idnt234 "hello" 76'
        tokens = tokenize(input_str)

        self.assertEqual(
            [
                Token(Type.Other, 'identifier'),
                Token(Type.Other, 'idnt234'),
                Token(Type.String, '"hello"'),
                Token(Type.Number, '76'),
            ],
            tokens
        )

    def test_empty_string(self):
        input_str = '""'
        tokens = tokenize(input_str)

        self.assertEqual(
            [Token(Type.String, '""')],
            tokens
        )

    def test_multiple_empty_strings(self):
        input_str = '"" "" ""'
        tokens = tokenize(input_str)

        self.assertEqual(
            [
                Token(Type.String, '""'),
                Token(Type.String, '""'),
                Token(Type.String, '""'),
            ],
            tokens
        )


if __name__ == "__main__":
    unittest.main()
