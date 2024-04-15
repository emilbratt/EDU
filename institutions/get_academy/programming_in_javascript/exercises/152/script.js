// oppgave A
//
function A() {
    let arr = [];
    for (i=100; i<1001; i+=7) {
        arr.push(i);
    }
    document.getElementById('app').innerHTML += `<div>A: ${arr}</div><br>`;
}

// oppgave B
//
function B() {
    let arr = [21, 15, 73, 49, 46];
    let total = summarizeArray(arr);
    document.getElementById('app').innerHTML += `<div>B: ${total}</div><br>`;
}

function summarizeArray(arr) {
    let total = 0;
    for (i=0; i<arr.length; i++) {
        total += arr[i];
    }
    return total;
}

// oppgave C
//
let Cnumbers = [2, 5, 7, 0, 3, 4, 1, 8, 6];

function C() {
    document.getElementById('app').innerHTML += '<div>C:</div><div id="taskC"></div><br>';
    drawNumbers();
}

function swapSquare(squareIndex, squareNumber) {
    let newCnumbers = [];
    for (i=0; i<Cnumbers.length; i++) {
        number = Cnumbers[i];
        if (i === squareIndex) {
            newCnumbers.push(0);
        } else if (number === 0) {
            newCnumbers.push(squareNumber);
        } else {
            newCnumbers.push(number);
        }
    }
    Cnumbers = newCnumbers;
    drawNumbers();
}

function drawNumbers() {
    let htmlTiles = '';
    for (index=0; index<Cnumbers.length; index++) {
        number = Cnumbers[index];
        htmlTiles += `<div onclick="swapSquare(${index}, ${number})" id="C${index}">`;
        if (number !== 0) {
            htmlTiles += number;
        }
        htmlTiles += '</div>';
    }
    document.getElementById('taskC').innerHTML = htmlTiles;
}

// Oppgave D
//
function D() {
    document.getElementById('app').innerHTML += `
        <div>D:</div>
        <div id="taskD">
            <textarea id="taskDtextarea"></textarea><br>
            <button onclick="countValidWords()">Tell ord</button>
            <div id="taskDresult">Resultat:</div>
        </div>
        <br>
    `;
}

function countValidWords() {
    const regex = new RegExp('^[a-åA-Å]+$');
    let foundWords = [];
    let wordCount = [];
    let text = document.getElementById('taskDtextarea').value;
    let wordList = text.split(' ');
    for (const word of wordList) {
        if (regex.test(word)) {
            if (foundWords.includes(word)) {
                wordCount[ foundWords.indexOf(word) ] += 1;
            } else {
                foundWords.push(word);
                wordCount.push(1);
            }
        }
    }
    let html = '';
    for (i=0; i<foundWords.length; i++) {
        html += `${foundWords[i]}: ${wordCount[i]}<br>`
    }
    document.getElementById('taskDresult').innerHTML = html;
}

// Oppgave E
//
function E() {
    document.getElementById('app').innerHTML += `
        <div>E:</div>
        <div id="taskE">
        </div>
        <br>
    `;

    let html = '';

    let list_unsorted = [2,5,8,1,3];
    let list_sorted   = [1,2,5,7,9];
    let res_unsorted = list_is_sorted(list_unsorted);
    let res_sorted = list_is_sorted(list_sorted);
    document.getElementById('taskE').innerHTML += `
        ${list_unsorted} -> ${res_unsorted} <br>
        ${list_sorted} -> ${res_sorted} <br>
    `;

}

function list_is_sorted(arr) {
    for (i=1; i<arr.length; i++) {
        if (arr[i-1]  > arr[i]) {
            return false;
        }
    }
    return true;
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
