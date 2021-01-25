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


if __name__ == "__main__":
    unittest.main()
