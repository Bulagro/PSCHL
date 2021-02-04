function readTextFile(file, callback) {
    var rawFile = new XMLHttpRequest();
    rawFile.responseType = 'json';
    rawFile.open("GET", file, true);

    rawFile.onreadystatechange = function() {
        if (rawFile.readyState === 4 && rawFile.status == 200) {
            callback(rawFile.response);
        }
    }
    rawFile.send(null);
}

function setSlider(element, value) {
    let r = 0, g = 0, b = 0;
    if (element.className == 'slider-red') {
        r = value;
    } else if (element.className == 'slider-green') {
        g = value;
    } else if (element.className == 'slider-blue') {
        b = value;
    }

    element.style.backgroundColor = `rgb(${r}, ${g}, ${b})`;
    element.value = value;
}

function setColorBar(bar, r, g, b) {
    let bar_color = `rgb(${r}, ${g}, ${b})`;
    bar.style.backgroundColor = bar_color;
}

function updateValues(theme) {
    readTextFile("https://bulagro.github.io/PSCHL/config/es.json", (config) => {
        let colorConfig = config;

        const elements = ['background', 'keyword', 'number', 'operator', 'string', 'delimiter', 'identifier', 'comment'];
        elements.forEach((e) => {
            let colorBar = document.getElementById(`${e}-color-display`);

            let sliderRed = document.getElementById(`${e}-red`);
            let sliderGreen = document.getElementById(`${e}-green`);
            let sliderBlue = document.getElementById(`${e}-blue`);

            if (e == 'background') {
                let r = colorConfig['themes'][theme]['background'][0],
                    g = colorConfig['themes'][theme]['background'][1],
                    b = colorConfig['themes'][theme]['background'][2];

                setSlider(sliderRed, r);
                setSlider(sliderGreen, g);
                setSlider(sliderBlue, b);
                setColorBar(colorBar, r, g, b);
            } else {
                let r = colorConfig['themes'][theme][e]['foreground'][0],
                    g = colorConfig['themes'][theme][e]['foreground'][1],
                    b = colorConfig['themes'][theme][e]['foreground'][2];

                setSlider(sliderRed, r);
                setSlider(sliderGreen, g);
                setSlider(sliderBlue, b);
                setColorBar(colorBar, r, g, b)
            }

            sliderRed.addEventListener('input', (_) => {
                let slider_color = `rgb(${sliderRed.value}, 0, 0)`;
                sliderRed.style.backgroundColor = slider_color;
                setColorBar(colorBar, sliderRed.value, sliderGreen.value, sliderBlue.value);
            });
            sliderGreen.addEventListener('input', (_) => {
                let slider_color = `rgb(0, ${sliderGreen.value}, 0)`;
                let bar_color = `rgb(${sliderRed.value}, ${sliderGreen.value}, ${sliderBlue.value})`;

                sliderGreen.style.backgroundColor = slider_color;
                colorBar.style.backgroundColor = bar_color;
            });
            sliderBlue.addEventListener('input', (_) => {
                let slider_color = `rgb(0, 0, ${sliderBlue.value})`;
                let bar_color = `rgb(${sliderRed.value}, ${sliderGreen.value}, ${sliderBlue.value})`;

                sliderBlue.style.backgroundColor = slider_color;
                colorBar.style.backgroundColor = bar_color;
            });
        });
    });
}

updateValues('light');
