import unittest, json
from tokenizer import *

with open('config/es.json', 'r') as f:
        keywords = json.load(f)['keywords']

class TokenizerTest(unittest.TestCase):
    def test_single_word(self):
        input_str = 'hello'
        tokens = tokenize(input_str, keywords)

        self.assertEqual(
            [Token(Type.Identifier, 'hello')],
            tokens
        )

    def test_sentence(self):
        input_str = 'These are a few words'
        tokens = tokenize(input_str, keywords)

        self.assertEqual(
            [
                Token(Type.Identifier, 'These'),
                Token(Type.Identifier, 'are'),
                Token(Type.Identifier, 'a'),
                Token(Type.Identifier, 'few'),
                Token(Type.Identifier, 'words'),
            ],
            tokens
        )

    def test_single_number(self):
        input_str = '234'
        tokens = tokenize(input_str, keywords)

        self.assertEqual(
            [Token(Type.Number, '234')],
            tokens
        )

    def test_numbers(self):
        input_str = '234 3 3 11'
        tokens = tokenize(input_str, keywords)

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
        tokens = tokenize(input_str, keywords)

        self.assertEqual(
            [Token(Type.Identifier, 'babt2002')],
            tokens
        )

    def test_multiple_identifiers(self):
        input_str = 'thes2 are24 ident1f13rs'
        tokens = tokenize(input_str, keywords)

        self.assertEqual(
            [
                Token(Type.Identifier, 'thes2'),
                Token(Type.Identifier, 'are24'),
                Token(Type.Identifier, 'ident1f13rs'),
            ],
            tokens
        )

    def test_mixed_numbers_and_identifiers(self):
        input_str = 'this string should have 6 elements'
        tokens = tokenize(input_str, keywords)

        self.assertEqual(
            [
                Token(Type.Identifier, 'this'),
                Token(Type.Identifier, 'string'),
                Token(Type.Identifier, 'should'),
                Token(Type.Identifier, 'have'),
                Token(Type.Number, '6'),
                Token(Type.Identifier, 'elements'),
            ],
            tokens
        )

    def test_identifier_that_start_with_numbers(self):
        input_str = '123onetwothree'
        tokens = tokenize(input_str, keywords)

        self.assertEqual(
            [Token(Type.Identifier, '123onetwothree')],
            tokens
        )

    def test_identifiers_that_start_with_numbers(self):
        input_str = '123onetwothree 2a'
        tokens = tokenize(input_str, keywords)

        self.assertEqual(
            [
                Token(Type.Identifier, '123onetwothree'),
                Token(Type.Identifier, '2a'),
            ],
            tokens
        )

    def test_single_string(self):
        input_str = ' "this is a string" '
        tokens = tokenize(input_str, keywords)

        self.assertEqual(
            [Token(Type.String, '"this is a string"')],
            tokens
        )

    def test_multiple_strings(self):
        input_str = '"a" "b" "c"'
        tokens = tokenize(input_str, keywords)

        self.assertEqual(
            [
                Token(Type.String, '"a"'),
                Token(Type.String, '"b"'),
                Token(Type.String, '"c"'),
            ],
            tokens
        )

    def test_strings_mixed_with_Identifier_types(self):
        input_str = 'identifier idnt234 "hello" 76'
        tokens = tokenize(input_str, keywords)

        self.assertEqual(
            [
                Token(Type.Identifier, 'identifier'),
                Token(Type.Identifier, 'idnt234'),
                Token(Type.String, '"hello"'),
                Token(Type.Number, '76'),
            ],
            tokens
        )

    def test_empty_string(self):
        input_str = '""'
        tokens = tokenize(input_str, keywords)

        self.assertEqual(
            [Token(Type.String, '""')],
            tokens
        )

    def test_multiple_empty_strings(self):
        input_str = '"" "" ""'
        tokens = tokenize(input_str, keywords)

        self.assertEqual(
            [
                Token(Type.String, '""'),
                Token(Type.String, '""'),
                Token(Type.String, '""'),
            ],
            tokens
        )

    def test_single_char_operator(self):
        input_str = '='
        tokens = tokenize(input_str, keywords)

        self.assertEqual(
            [Token(Type.Operator, '=')],
            tokens
        )

    def test_multiple_single_char_operators(self):
        input_str = '= + -'
        tokens = tokenize(input_str, keywords)

        self.assertEqual(
            [
                Token(Type.Operator, '='),
                Token(Type.Operator, '+'),
                Token(Type.Operator, '-'),
            ],
            tokens
        )

    def test_single_multichar_operator(self):
        input_str = '=='
        tokens = tokenize(input_str, keywords)

        self.assertEqual(
            [Token(Type.Operator, '==')],
            tokens
        )

    def test_multiple_multichar_operators(self):
        input_str = '== -= !='
        tokens = tokenize(input_str, keywords)

        self.assertEqual(
            [
                Token(Type.Operator, '=='),
                Token(Type.Operator, '-='),
                Token(Type.Operator, '!='),
            ],
            tokens
        )

    def test_combined_uni_and_multichar_operators(self):
        input_str = '= >= ++ !'
        tokens = tokenize(input_str, keywords)

        self.assertEqual(
            [
                Token(Type.Operator, '='),
                Token(Type.Operator, '>='),
                Token(Type.Operator, '++'),
                Token(Type.Operator, '!'),
            ],
            tokens
        )

    def test_multichar_operators_can_only_be_two_characters_long(self):
        input_str = '==='
        tokens = tokenize(input_str, keywords)

        self.assertEqual(
            [
                Token(Type.Operator, '=='),
                Token(Type.Operator, '='),
            ],
            tokens
        )

    def test_combined_identifiers_and_operators(self):
        input_str = 'hello + bye'
        tokens = tokenize(input_str, keywords)

        self.assertEqual(
            [
                Token(Type.Identifier, 'hello'),
                Token(Type.Operator, '+'),
                Token(Type.Identifier, 'bye'),
            ],
            tokens
        )

    def test_minus_operator_before_number_becomes_part_of_number_if_theres_no_space_between_them(self):
        input_str = '-23'
        tokens = tokenize(input_str, keywords)

        self.assertEqual(
            [Token(Type.Number, '-23')],
            tokens
        )

    def test_minus_operator_doesnt_become_part_of_number_if_separated(self):
        input_str = '-48234 - 5'
        tokens = tokenize(input_str, keywords)

        self.assertEqual(
            [
                Token(Type.Number, '-48234'),
                Token(Type.Operator, '-'),
                Token(Type.Number, '5'),
            ],
            tokens
        )

    def test_newlines(self):
        input_str = 'this \n should \n have \n many \n newlines'
        tokens = tokenize(input_str, keywords)

        self.assertEqual(
            [
                Token(Type.Identifier, 'this'),
                Token(Type.NewLine, None),
                Token(Type.Identifier, 'should'),
                Token(Type.NewLine, None),
                Token(Type.Identifier, 'have'),
                Token(Type.NewLine, None),
                Token(Type.Identifier, 'many'),
                Token(Type.NewLine, None),
                Token(Type.Identifier, 'newlines'),
            ],
            tokens
        )

    def test_delimiters(self):
        input_str = 'bar(foo(1 + 2))'
        tokens = tokenize(input_str, keywords)

        self.assertEqual(
            [
                Token(Type.Identifier, 'bar'),
                Token(Type.Delimiter, '('),
                Token(Type.Identifier, 'foo'),
                Token(Type.Delimiter, '('),
                Token(Type.Number, '1'),
                Token(Type.Operator, '+'),
                Token(Type.Number, '2'),
                Token(Type.Delimiter, ')'),
                Token(Type.Delimiter, ')'),
            ],
            tokens
        )

    def test_number_with_decimal_point(self):
        input_str = '0.1'
        tokens = tokenize(input_str, keywords)

        self.assertEqual(
            [Token(Type.Number, '0.1')],
            tokens
        )

    def test_multiple_numbers_with_decimal_point(self):
        input_str = '0.1 53.3 7646.21415'
        tokens = tokenize(input_str, keywords)

        self.assertEqual(
            [
                Token(Type.Number, '0.1'),
                Token(Type.Number, '53.3'),
                Token(Type.Number, '7646.21415'),
            ],
            tokens
        )

    def test_mixed_point_numbers(self):
        input_str = '0.1 10'
        tokens = tokenize(input_str, keywords)

        self.assertEqual(
            [
                Token(Type.Number, '0.1'),
                Token(Type.Number, '10'),
            ],
            tokens
        )


    def test_negative_floating_point_number(self):
        input_str = '-0.1'
        tokens = tokenize(input_str, keywords)

        self.assertEqual(
            [Token(Type.Number, '-0.1')],
            tokens
        )

    def test_es_keywords(self):
        input_str = 'si entonces finpara'
        tokens = tokenize(input_str, keywords)

        self.assertEqual(
            [
                Token(Type.OKeyword, 'si'),
                Token(Type.Keyword, 'entonces'),
                Token(Type.CKeyword, 'finpara'),
            ],
            tokens
        )

    def test_es_keywords_capitalized(self):
        input_str = 'FinPara Mientras'
        tokens = tokenize(input_str, keywords)

        self.assertEqual(
            [
                Token(Type.CKeyword, 'FinPara'),
                Token(Type.OKeyword, 'Mientras'),
            ],
            tokens
        )

    def test_multiline_strings(self):
        input_str = ''' "a
        "'''
        tokens = tokenize(input_str, keywords)

        self.assertEqual(
            [Token(Type.String, '"a        "')],
            tokens
        )


if __name__ == "__main__":
    unittest.main()
