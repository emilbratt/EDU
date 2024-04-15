// lib
//
function validateHeight() {
    if (barSelectedHeight > BAR_MAX_HEIGHT) {
        alert(`Feil: ${barSelectedHeight} er for høy`);
        return false;
    } else if (barSelectedHeight < BAR_MIN_HEIGHT) {
        alert(`Feil: ${barSelectedHeight} er for lav`);
        return false;
    }
    return true;
}

function generateBar(barIndex, barHeight, total) {
    let colorHue = 220 - ((90 / total) * barIndex);
    let htmlClass = '';
    if (barSelectedActivate && barSelectedIndex === barIndex) {
        htmlClass += 'selectedBar';
    }
    // SVG bars are generated based on specific attributes inside the <rect> tag
    return `
        <rect id="bar${barIndex}"
              class="${htmlClass}"
              width="${8}"
              height="${barHeight-2}"
              x="${barIndex*11}"
              y="${(-barHeight)+(61)}"
              onclick="selectedSVGBar(${barIndex})"
              fill="hsl(${colorHue}, 100%, 50%)" />
    `;
}

function generateNumberInput(min, max, val, oninput) {
    return `<input type="number" min="${min}" max="${max}" value="${val}" oninput="${oninput}"/>`;
}

function generateButton(text, onclick, isEnabled) {
    if (isEnabled) {
        return `<button onclick="${onclick}">${text}</button>`;
    } else {
        return `<button disabled onclick="${onclick}">${text}</button>`;
    }
}
