function fixText(text) {
    let finalText = '';
    let upperCaseAdd = true;
    for (let i=0; i<text.length; i++) {
        let char = text[i];
        if (char !== ' ') {
            char = upperCaseAdd ? char.toUpperCase() : char.toLowerCase();
            finalText += char;
            upperCaseAdd = false;
        }
    }
    return finalText;
}

function isEmail(text) {
    let arrExpectedChars = ['.', '@', '.'];
    let index = 0;
    for (let i=0; i<text.length; i++) {
        let char = text[i];
        if (char === arrExpectedChars[index]) {
            index += 1;
        }
    }
    return index === 3;
}
