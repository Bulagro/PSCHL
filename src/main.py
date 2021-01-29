from PIL import Image, ImageDraw, ImageFont
import json

from tokenizer import *
from formatter import *


def draw_word(x, y, font, color, word, img_draw, char_spacing):
    for l in word:
        img_draw.text((x, y), l, fill=color, font=font)
        x += char_spacing

    return x + char_spacing


def generate_image(input_str):
    with open('config/es.json', 'r') as f:
        config = json.load(f)

    lines = get_lines(
        tokenize(input_str, config['keywords'])
    )

    font = ImageFont.truetype('config/font/FiraCode-Regular.ttf', size=14)
    image = Image.new('RGB', (500, 400), (255, 255, 255))
    draw = ImageDraw.Draw(image)

    x_spacing = 2 * font.size / 3
    y_spacing = font.size + 2
    tab_size = 4

    y = 0
    for line in lines:
        x = line.indent * x_spacing * tab_size

        for token in line.tokens:
            r, g, b = config['colors'][token.type.value]['foreground']
            color = f'rgb({r}, {g}, {b})'

            x = draw_word(
                x, y,
                font,
                color,
                token.content,
                draw,
                x_spacing
            )

        y += y_spacing

    image.save('something.png')

generate_image('''si a entonces
        b + 1
        si b entonces
        a - 1
        finsi
        finsi
 ''')
