// oppgave A
//
function drawRGBDiv(rgb) {
    document.getElementById('A').innerHTML = '<div id="colorBox"></div>';
    box = document.getElementById('colorBox');
    box.style.width = '50px';
    box.style.height = '50px';
    box.style.backgroundColor = `rgb(${rgb.red}, ${rgb.green}, ${rgb.blue})`;
}

function A() {
    let currentColor = {red: 100, green: 200, blue: 50};
    drawRGBDiv(currentColor);
}


// oppgave B
//
function invertRGB(rgb) {
    return {
        red: 255-rgb.red,
        green: 255-rgb.green,
        blue: 255-rgb.blue,
    };
}

function drawRGBDivAndIncludeInverseRGB(rgb) {
    document.getElementById('B').style.display = 'flex';
    document.getElementById('B').innerHTML = '<div style="width: 50px; height: 50px;" id="colorBoxNormal"></div>';
    document.getElementById('B').innerHTML += '<div style="width: 50px; height: 50px;" id="colorBoxInverse"></div>';
    document.getElementById('colorBoxNormal').style.backgroundColor = `rgb(${rgb.red}, ${rgb.green}, ${rgb.blue})`;
    rgb = invertRGB(rgb);
    document.getElementById('colorBoxInverse').style.backgroundColor = `rgb(${rgb.red}, ${rgb.green}, ${rgb.blue})`;
}

function B() {
    let currentColor = {red: 50, green: 50, blue: 50};
    drawRGBDivAndIncludeInverseRGB(currentColor);
}


// oppgave C
//
function meanChannel(cA, cB) {
    return (cA + cB) / 2;
}

function meanRGB(rgbA, rgbB) {
    return {
        red: meanChannel(rgbA.red, rgbB.red),
        green: meanChannel(rgbA.green, rgbB.green),
        blue: meanChannel(rgbA.blue, rgbB.blue),
    };
}

function drawRGBDivMean(rgbA, rgbB) {
    let rgb = meanRGB(rgbA, rgbB);
    document.getElementById('C').innerHTML = '<div id="colorBoxMean"></div>';
    box = document.getElementById('colorBoxMean');
    box.style.width = '50px';
    box.style.height = '50px';
    box.style.backgroundColor = `rgb(${rgb.red}, ${rgb.green}, ${rgb.blue})`;
}

function C() {
    let rgbA = {red: 250, green: 50, blue: 5};
    let rgbB = {red: 150, green: 150, blue: 15};
    drawRGBDivMean(rgbA, rgbB);
}


// oppgave D
//
function gradientRGB(rgb) {
    // Farge 1 skal være parameteren.
    let rgb1 = rgb;

    // Farge 5 skal være den inverterte fargen.
    let rgb5 = invertRGB(rgb);

    // Farge 3 skal være gjennomsnittet av farge 1 og 5.
    let rgb3 = meanRGB(rgb1, rgb5);

    // Farge 2 skal være gjennomsnittet av farge 1 og farge 3
    let rgb2 = meanRGB(rgb1, rgb3);

    // Farge 4 skal være gjennomsnittet av farge 3 og farge 5.
    let rgb4 = meanRGB(rgb3, rgb5);

    return [rgb1, rgb2, rgb3, rgb4, rgb5];
}

function drawRGBDivGradient(rgb) {
    document.getElementById('D').style.display = 'flex';
    rgbGradient = gradientRGB(rgb);
    for (let i=0; i<5; i++) {
        let c = rgbGradient[i];
        let id = `colorBoxGradient${i}`;
        document.getElementById('D').innerHTML += `<div id="${id}"></div>`;
        let box = document.getElementById(id);
        box.style.width = '50px';
        box.style.height = '50px';
        box.style.backgroundColor = `rgb(${c.red}, ${c.green}, ${c.blue})`;
    }
}

function D() {
    let rgb = {red: 250, green: 50, blue: 15};
    drawRGBDivGradient(rgb);
}


// oppgave E
//
function getWordStats() {
    let html = 'Statistikk';
    let text = document.getElementById('textareaTaskE').value;
    let maxLength = 0;
    let wordCounts = {};
    let wordlist = text.split(" ");
    for (let i=0; i<wordlist.length; i++) {
        let word = wordlist[i];
        if (word.length > maxLength) {
            maxLength = word.length;
        }
        if (wordCounts[word]) {
            wordCounts[word]++;
        } else {
            wordCounts[word] = 1;
        }
    }
    html += '<pre>';
    for (let word in wordCounts) {
        html += `${word.padStart(maxLength, ' ')}: ${wordCounts[word]}\n`;
    }
    html += '</pre>'
    document.getElementById('resultTaskE').innerHTML = html;

}

function E() {
    document.getElementById('E').innerHTML = 'hei';
    let html = `
        <div id="taskD">
            <textarea id="textareaTaskE"></textarea><br>
            <button onclick="getWordStats()">Send</button>
            <div id="resultTaskE">Statistikk:</div>
        </div>
    `;
    document.getElementById('E').innerHTML = html;
}


// entrypoint
//
function main() {
    A();
    B();
    C();
    D();
    E();
}
main();
