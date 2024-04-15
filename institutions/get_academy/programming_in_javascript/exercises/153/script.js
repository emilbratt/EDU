// model
const maxPages = 100;
const pageSpan = 10;
let currentPage = 1;
let filterRule = 'standard';

// view
//
function updateView() {
    let html = '<h1>Oppgave 153</h1>';
    html += updateViewSelectFilter();
    html += updateViewSelectPages();
    document.getElementById('app').innerHTML = html;
    return html;
}

function updateViewSelectPages() {
    let html = `<p>Selected page: ${currentPage}</p>`;
    let pageSelection = getPageSelectionList();
    html += getPageSelectionHTML(pageSelection);
    return html;
}

function updateViewSelectFilter() {
    let html = `<p>Filter selection</p>`;
    html += `<p>Current filter: ${filterRule}</p>`;
    html += `<button class="filterSelection" onclick="applyFilter('standard')">Standard</button>`;
    html += `<button class="filterSelection" onclick="applyFilter('odd')">Odd</button>`;
    html += `<button class="filterSelection" onclick="applyFilter('even')">Even</button>`;
    return html;

}

// controller
//
function choosePage(n) {
    currentPage = n;
    updateView();
}

function applyFilter(rule) {
    filterRule = rule;
    updateView();
}


// lib
function getPageSelectionList() {
    let pageNumberSelection = [];
    let from = currentPage > maxPages-pageSpan + 2 ? maxPages-pageSpan
             : currentPage < 5 ? 0
             : currentPage -5;

    for (let i=from; i<=from+pageSpan; i++) {
        pageNumberSelection.push(i);
    }
    console.log(from);
    return pageNumberSelection;
}

function getPageSelectionHTML(pageSelection) {
    let html = '';
    let color;
    let fCall;
    let addButton;
    for (let i=1; i<pageSelection.length; i++) {
        let n = pageSelection[i];
        if (currentPage === n) {
            color = 'gray';
            fCall = '';
        } else {
            color = 'black';
            fCall = `choosePage(${n})`;
        }
        if (filterRule === 'standard') {
            addButton = true;
        } else if (filterRule === 'odd') {
            addButton = n % 2 == 1 ? true : false;
        } else if (filterRule === 'even') {
            addButton = n % 2 == 0 ? true : false;
        }
        if (addButton) {
            html += `<button class="pageSelection" onclick="${fCall}" style="color: ${color};">${n}</button>`;
        }
    }
    return html;
}


// entrypoint
function main() {
    updateView();
}
main();
