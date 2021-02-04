let colorConfig = null; // Get json

const elements = ['background', 'keyword', 'number', 'operator', 'string', 'delimiter', 'identifier', 'comment'];
elements.forEach((e, _) => {
    let colorBar = document.getElementById(`${e}-color-display`);

    let sliderRed = document.getElementById(`${e}-red`);
    let sliderGreen = document.getElementById(`${e}-green`);
    let sliderBlue = document.getElementById(`${e}-blue`);

    sliderRed.addEventListener('input', (_) => {
        let slider_color = `rgb(${sliderRed.value}, 0, 0)`;
        let bar_color = `rgb(${sliderRed.value}, ${sliderGreen.value}, ${sliderBlue.value})`;

        sliderRed.style.backgroundColor = slider_color;
        colorBar.style.backgroundColor = bar_color;
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
