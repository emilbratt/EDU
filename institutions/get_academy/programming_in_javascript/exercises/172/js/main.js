function addDummyData() {
    function _dummyAddTodo(checked) {
        let todo = {
            id: todoIndex,
            active: true,
            text: editTodoText,
            responsible: editTodoResponsible,
            doneDate: checked ? getCompletedDate() : null, // ISO-timestamp as string if done
        };
        todoIndex += 1;
        todoList.push(todo);
    }
    editTodoResponsible = 'Kim';
    editTodoText = 'vaske';
    _dummyAddTodo(checked=false);

    editTodoResponsible = 'Morten';
    editTodoText = 'kjøpe hylle';
    _dummyAddTodo(checked=true);

    editTodoResponsible = 'Kim';
    editTodoText = 'støvsuge';
    _dummyAddTodo(checked=false);

    editTodoResponsible = 'Ida';
    editTodoText = 'henge opp bilder';
    _dummyAddTodo(checked=false);

    todoList[3].active = false;
    editTodoResponsible = 'Morten';
    editTodoText = 'sette opp hylle';
    _dummyAddTodo(checked=true);

    editTodoResponsible = 'Ida';
    editTodoText = 'vanne blomster';
    _dummyAddTodo(checked=false);

    editTodoResponsible = 'Ida';
    editTodoText = 'gå tur med hun';
    _dummyAddTodo(checked=false);

    editTodoResponsible = 'Ida';
    editTodoText = 'bake boller';
    _dummyAddTodo(checked=false);

    editTodoResponsible = 'Morten';
    editTodoText = 'si opp netflix ab.';
    _dummyAddTodo(checked=false);

    editTodoResponsible = 'Morten';
    editTodoText = 'send blomster til mor';
    _dummyAddTodo(checked=true);

    editTodoResponsible = 'Kim';
    editTodoText = 'legg tv ut for salg';
    _dummyAddTodo(checked=false);

    editTodoResponsible = 'Kim';
    editTodoText = 'søk på stipend';
    _dummyAddTodo(checked=false);

    editTodoText = '';
    editTodoResponsible = '';

}


// entrypoint
//
function main() {
    if (ADD_DUMMY_DATA) {
        addDummyData();
    }
    updateView();
}
main();
