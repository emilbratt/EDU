function getCompletedDate() {
    let d = new Date();
    return `${d.getFullYear()}/${d.getMonth()+1}/${d.getDate()}`;
}

function resetEditIndexes() {
    editTodoIndexText = -1;
    editTodoIndexResponsible = -1;
}

function prepareData() {
    // Builds the array of indexes to all todo lists which will be rendered.
    // Builds the array for all page numbers form 1 to N based on what is inside showIndexes.

    // showIndexes -> what todo lists to include for rendering
    showIndexes.length = 0; // reset array on every refresh
    for (let i=0; i<todoIndex; i++) {
        let todo = todoList[i];
        let add = true;
        if (!matchFilterResponsible(todo)) {
            add = false;
        } else if (!matchFilterStatus(todo)) {
            add = false;
        } else if (!todo.active && !isShowDeleted) {
            add = false;
        }
        if (add) {
            showIndexes.push(i);
        }
    }
    if (!paginationEnabled) {
        return;
    }
    // array for all page numbers based on showIndexes and also builds a new "currentPageIndexes"
    let currentPageIndexes = []; // filter out only indexes from the selected page
    let pageNumber = 1;
    let i = 0;
    let ignoreLastPage = false;
    pageNumbers = [1]; // each page button will get its own number from this list..
    for (const index of showIndexes) {
        i += 1;
        if (pageNumber === selectedPage) {
            currentPageIndexes.push(index);
        }
        if (i === paginationRowLimit) {            
            pageNumber += 1;
            i = 0;
            pageNumbers.push(pageNumber);
        }
        // if true on last iteration, there will be no more entries added to the new page.
        ignoreLastPage = (i % paginationRowLimit === 0) ? true : false;
    }
    // if no entries where added after a new page was added, pop the last page number
    if (ignoreLastPage) {
        pageNumbers.pop();
    }

    // Only include indexes based on what page is selected.
    showIndexes = currentPageIndexes;
}

function buildPaginationRow() {
    let html = '';
    for (const page of pageNumbers) {
        let _class = parseInt(page) === selectedPage ? 'class="paginationActiveColor"' : '';
        html += `<button onclick="buttonSelectPage(${page})" ${_class}>${page}</button>`;
    }
    return html;
}

function buildTodoListRows() {
    let rows = '';
    for (let i=0; i<showIndexes.length; i++) {
        index = showIndexes[i];
        rows += buildTodoListColumns(todoList[index]);
    }
    return rows;
}

function buildTodoListColumns(todo) {
    let color = todo.active === false ? 'deletedColor'
                        : todo.doneDate !== null ? 'checkedColor'
                        : colorSwitchRow === true ? 'uncheckedColor1'
                        : 'uncheckedColor2';

    colorSwitchRow = colorSwitchRow ? false : true;
    return `
        <tr class="${color}">
            <td style="${WITDH_COL_A}">${buildTodoListColumnText(todo)}</td>
            <td style="${WITDH_COL_B}">${buildTodoListColumnResponsible(todo)}</td>
            <td style="${WITDH_COL_C}">${buildTodoListColumnDoneDate(todo)}</td>
        </tr>
    `;
}

function buildTodoListColumnText(todo) {
    let html = `
        ${todo.text}
        <button style="float: right;"
                onclick="editTodo('text', ${todo.id})">Rediger
        </button>
    `;

    if (todo.id === editTodoIndexText) {
        editTodoText = todo.text; // this line is needed to avoid ampty string if no changes where made
        html = `
            <input autofocus
                   style="width: 95%;"
                   onchange="applyTodo('text', ${todo.id})"
                   onkeydown="if (event.code === 'Enter') applyTodo('text', ${todo.id});"
                   oninput="oninputUpdateTodoField('text', this.value)"
                   type="text"
                   value="${todo.text}" />
        `;
    }
    return html;
}

function buildTodoListColumnResponsible(todo) {
    let responsibleCol = `
        ${todo.responsible}
        <button style="float: right;"
                onclick="editTodo('responsible', ${todo.id})">Rediger
        </button>
    `;

    if (todo.id === editTodoIndexResponsible) {
        editTodoResponsible = todo.responsible; // this line is needed to avoid ampty string if no changes where made
        responsibleCol = `
            <input autofocus
                   style="width: 93%;"
                   onchange="applyTodo('responsible', ${todo.id})"
                   onkeydown="if (event.code === 'Enter') applyTodo('responsible', ${todo.id});"
                   oninput="oninputUpdateTodoField('responsible', this.value)"
                   type="text"
                   value="${todo.responsible}" />
        `;
    }
    return responsibleCol;
}

function buildTodoListColumnDoneDate(todo) {
    let doneDateCol = todo.active ?
        `<button style="float: right;" onclick="applyTodo('active', ${todo.id})">Slett</button>` :
        `<button style="float: right;" onclick="applyTodo('active', ${todo.id})">Gjennopprett</button>`;

    doneDateCol += todo.active && todo.doneDate === null ?
        `<input onclick="applyTodo('doneDate', ${todo.id})"
                type="checkbox" />
        <label>Gjenstår</label>`
    : todo.active && todo.doneDate !== null ?
        `<input checked
                onclick="applyTodo('doneDate', ${todo.id})"
                type="checkbox" />
        <label>${todo.doneDate}</label>`
    : '';
    return doneDateCol;
}

function buildTodoFilterRow() {
    return `
        <tr class="headerColor">

            <th style="${WITDH_COL_A}">
                <div style="float: left;">
                    Oppgave
                </div>
            </th>

            <th style="${WITDH_COL_B}">
                <div style="float: left;">
                    Person
                </div>
                <div style="float: right;">
                    <select style="width: 100%;" onchange="onchangeFilterBy('responsible', this.value)">
                        ${buildTodoFilterColumnResponsible()}
                    </select>
                </div>
            </th>

            <th style="${WITDH_COL_C}">
                <div style="float: left;">
                    Status
                </div>
                <div style="float: right;">
                    <select style="width: 100%;" onchange="onchangeFilterBy('doneDate', this.value)">
                        ${buildTodoFilterColumnDoneDate()}
                    </select>
                </div>
            </th>

        </tr>
    `;
}

function buildTodoFilterColumnResponsible() {
    // This function adds the selected item as the first option
    // because selected options are only applied by the event "onchange".
    // This means having the selected option the first entry makes it unselectable
    // while all the others that come after will remain selectable.
    let html = '';
    if (filterByResponsible === '') {
        html += '<option value="">-Vis alle-</option>'; // always show this as 1st option (non selectable)
    } else {
        html += `<option value="${filterByResponsible}">${filterByResponsible}</option>`; // always show this as 1st option (non selectable)
        html += '<option value="">-Vis alle-</option>'; // always show this as 2nd option
    }

    let addedOption = [filterByResponsible];
    for (let i=0; i<todoIndex; i++) {
        let option = todoList[i].responsible;
        if (!addedOption.includes(option)) {
            if (todoList[i].active) {
                html += `<option value="${option}">${option}</option>`;
                addedOption.push(option);
            } else if (isShowDeleted) {
                html += `<option value="${option}">${option}</option>`;
                addedOption.push(option);
            }
        }
    }
    return html;
}

function buildTodoFilterColumnDoneDate() {
    // This function adds the selected item as the first option
    // because selected options are only applied by the event "onchange".
    // This means having the selected option the first entry makes it unselectable
    // while all the others that come after will remain selectable.
    let html = '';
    if (filterByDoneDate === '') {
        html += '<option value="">-Vis alle-</option>'; // always show this as 1st option (non selectable)
    } else {
        html += `<option value="${filterByDoneDate}">${filterByDoneDate}</option>`; // always show this as 1st option (non selectable)
        html += '<option value="">-Vis alle-</option>'; // always show this as 2nd option
    }

    let addedOption = [filterByDoneDate];
    for (let i=0; i<todoIndex; i++) {
        let option = getStatusReference(todoList[i])
        if (!addedOption.includes(option)) {
            if (todoList[i].active) {
                html += `<option value="${option}">${option}</option>`;
                addedOption.push(option);
            } else if (isShowDeleted) {
                html += `<option value="${option}">${option}</option>`;
                addedOption.push(option);
            }
        }
    }
    return html;
}

function getStatusReference(todo) {
    // based on several conditions, return the appriopriate status
    return todo.active === false ? STATUS_REFERENCE.deleted
        : todo.doneDate === null ? STATUS_REFERENCE.unchecked
        : STATUS_REFERENCE.checked;
}

function matchFilterResponsible(todo) {
    if (filterByResponsible === '') {
        return true;
    }
    return filterByResponsible === todo.responsible;
}

function matchFilterStatus(todo) {
    // based on several conditions, return if the todo matches the filter
    return filterByDoneDate === '' ? true
        : todo.active === false && filterByDoneDate === STATUS_REFERENCE.deleted ? true
        : todo.doneDate === null && filterByDoneDate === STATUS_REFERENCE.unchecked ? true
        : todo.doneDate !== null && filterByDoneDate === STATUS_REFERENCE.checked ? true
        : false;
}
