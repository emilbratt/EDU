function addTodo() {
    if (editTodoText === '' || editTodoResponsible === '') {
        editTodoText = '';
        editTodoResponsible = '';
        updateView();
        return;
    }
    let todo = {
        id: todoIndex,
        active: true,
        text: editTodoText,
        responsible: editTodoResponsible,
        doneDate: null, // ISO-timestamp as string if done
    };
    todoIndex += 1;
    todoList.push(todo);
    editTodoText = '';
    editTodoResponsible = '';
    updateView();
}

function buttonSelectPage(n) {
    selectedPage = n;
    updateView();
}

function editTodo(field, id) {
    resetEditIndexes(); // ensure no other field is already active
    switch (field) {
        case 'text':
            editTodoIndexText = id;
            break;
        case 'responsible':
            editTodoIndexResponsible = id;
            break;
    }
    updateView();
}

function applyTodo(field, id) {
    switch (field) {
        case 'active':
            todoList[id].active = todoList[id].active ? false : true;
            break;
        case 'text':
            todoList[id].text = editTodoText;
            editTodoText = '';
            break;
        case 'responsible':
            todoList[id].responsible = editTodoResponsible;
            editTodoResponsible = '';
            break;
        case 'doneDate':
            todoList[id].doneDate = todoList[id].doneDate === null ? getCompletedDate() : null;
            break;
    }
    resetEditIndexes();
    updateView();
}

function oninputUpdateTodoField(field, value) {
    // This is called by onchange and oninput events.
    // We do not want to call updateView() everytime because that makes the cursor jump out of the input field.
    switch (field) {
        case 'text':
            editTodoText = value;
            break;
        case 'responsible':
            editTodoResponsible = value;
            break;
    }
}

function onchangeFilterBy(field, value) {
    selectedPage = 1; // when adding a filter, the total page numbers change, lets reset this
    switch (field) {
        case 'responsible':
            filterByResponsible = value;
            break;
        case 'doneDate':
            filterByDoneDate = value;
            break;
    }
    updateView();
}

function toggleSetting(setting, value) {
    switch (setting) {
        case 'isShowDeleted':
            isShowDeleted = isShowDeleted ? false : true;
            break;
        case 'paginationEnabled':
            paginationEnabled = paginationEnabled ? false : true;
            break;
        case 'paginationRowLimit':
            paginationRowLimit = parseInt(value);
            break;
    }
    selectedPage = 1; // when adding a filter, the total page numbers change, lets reset this
    updateView();
}
