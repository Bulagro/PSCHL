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


if __name__ == "__main__":
    unittest.main()
