function updateView() {
    // build lists based on page selection and filter options
    prepareData(); // updates the model variables: showIndexes & pageNumbers

    // generate the html
    let html = '';
    html += updateViewSettings();
    html += updateViewAddTodo();
    if (paginationEnabled) {
        html += updateViewPaging();
    }
    html += updateViewShowTodo();
    document.getElementById('app').innerHTML = html;
}

function updateViewAddTodo() {
    return `
        <div class="todoAddBox">

            <label>Oppgave</label>
            <textarea style="height: 3rem;"
                      oninput="oninputUpdateTodoField('text', this.value)"
                      ></textarea>

            <label>Person</label>
            <input onkeydown="if (event.code === 'Enter') addTodo()"
                   oninput="oninputUpdateTodoField('responsible', this.value)"
                   type="text" />

            <button style="margin-top: 0.5rem; width: 30%; height: 2rem;"
                    onclick="addTodo()"
                    >Legg til</button>

        </div>
    `;
}

function updateViewSettings() {
    let html = '<div style="float: right; margin-top: 2rem;">';
    let colorShowDeleted = isShowDeleted ? 'settingEnableColor' : '';
    let colorEnablePagination = paginationEnabled ? 'settingEnableColor' : '';

    html += `
        <button style="width: 9rem; height: 2rem;"
                class="${colorShowDeleted}"
                onclick="toggleSetting('isShowDeleted', ${isShowDeleted})"
                >Vis slettede</button><br>
    `;
    html += `
        <button style="width: 9rem; height: 2rem;"
                class="${colorEnablePagination}"
                onclick="toggleSetting('paginationEnabled', ${paginationEnabled})"
                >Aktiver paginering</button><br>
    `;
    if (paginationEnabled) {
        html += `
            <label>Antall rader</label>
            <input style="width: 2rem;"
                    onchange="toggleSetting('paginationRowLimit', this.value)"
                    type="number"
                    min="1"
                    max="${todoIndex+1}"
                    value="${paginationRowLimit}" />
        `;
    }
    html += '</div>';
    return html;
}

function updateViewPaging() {
    return `
        <div class="pagination">
            ${buildPaginationRow()}
        </div>
    `;
}

function updateViewShowTodo() {
    colorSwitchRow = true;
    return `
        <table>
            ${buildTodoFilterRow()}
            ${buildTodoListRows()}
        </table>
    `;
}

/*

*/
