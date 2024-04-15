// view
//
function updateView() {
    let html = '';
    html += updateViewBarSVG();
    html += updateViewSelectedBar();
    html += updateViewSelectHeight();
    html += updateViewAddBarButton();
    html += updateViewChangeBarButton();
    html += updateViewRemoveBarButton();
    document.getElementById('content').innerHTML = html;
}

function updateViewBarSVG() {
    let bars = '';
    let barCount = barArray.length;
    for (let barIndex=0; barIndex<barCount; barIndex++) {
        barHeight = barArray[barIndex]*6;
        bars += generateBar(barIndex, barHeight, barCount);
    }
    let svg = '';
    svg += '<svg id="chart" width="500" viewBox="0 0 110 60">';
    svg += bars;
    svg += '</svg><br/>';
    return svg;
}

function updateViewSelectedBar() {
    let html = '';
    let selected = ' ingen';
    if (barSelectedActivate) {
        selected = String(barSelectedIndex+1);
    }
    html += `<label>Valgt stolpe: </i>${selected}</label><br>`;
    return html;
}

function updateViewSelectHeight() {
    let html = '<label>Valgt høyde: </label>';
    html +=  generateNumberInput(
        BAR_MIN_HEIGHT,
        BAR_MAX_HEIGHT,
        barSelectedHeight,
        'barSelectedHeight = this.value'
    );
    return html;
}

function updateViewAddBarButton() {
    let isEnabled = (barArray.length !== BAR_TOTAL_LIMIT);
    let text = 'Legg til stolpe';
    let onclick = 'numberInputAddBar()';
    return generateButton(text, onclick, isEnabled);
}

function updateViewChangeBarButton() {
    let text = 'Endre valgt stolpe';
    let onclick = `selectedBarChange(${barSelectedIndex})`;
    return generateButton(text, onclick, barSelectedActivate);
}

function updateViewRemoveBarButton() {
    let text = 'Fjern valgt stolpe';
    let onclick = `selectedBarRemove(${barSelectedIndex})`;
    return generateButton(text, onclick, barSelectedActivate);
}
