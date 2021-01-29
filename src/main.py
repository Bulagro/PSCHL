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
    with open('config/general.json', 'r') as f:
        config = json.load(f)

    with open('config/' + config['lang'] + '.json', 'r') as f:
        config.update(json.load(f))

    lines = get_lines(
        tokenize(input_str, config['keywords'])
    )

    font = ImageFont.truetype('config/font/FiraCode-Regular.ttf', size=config['font-size'])
    image = Image.new('RGB', (500, 400), (255, 255, 255))
    draw = ImageDraw.Draw(image)

    x_spacing = config['x-spacing']
    y_spacing = config['y-spacing']
    tab_size = config['tab-size']

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
