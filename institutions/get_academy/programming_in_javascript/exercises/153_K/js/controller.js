// controller
//
function selectedSVGBar(n) {
    if (barSelectedActivate && barSelectedIndex === n) {
        barSelectedActivate = false;
    } else if (!barSelectedActivate && barSelectedIndex === n) {
        barSelectedActivate = true;
    } else {
        barSelectedIndex = n;
        barSelectedActivate = true;
    }
    updateView();
}

function selectedBarChange() {
    if (validateHeight()) {
        let newBarArray = [];
        for (i=0; i<barArray.length; i++) {
            if (i === barSelectedIndex) {
                newBarArray.push(barSelectedHeight)
            } else {
                oldBar = barArray[i];
                newBarArray.push(oldBar);
            }
            newBarArray.push
        }
        barArray = newBarArray;
        barSelectedActivate = false;
    }
    updateView();
}

function selectedBarRemove() {
    barArray.splice(barSelectedIndex, 1);
    barSelectedActivate = false;
    updateView();
}

function numberInputAddBar() {
    if (validateHeight()) {
        barArray.push(barSelectedHeight)
    }
    updateView();
}
