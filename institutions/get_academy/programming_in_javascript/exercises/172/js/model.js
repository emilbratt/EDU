// add dummy data for testing etc.
const ADD_DUMMY_DATA = true;

// fixed column width
const WITDH_COL_A = 'width: 40%;';
const WITDH_COL_B = 'width: 29%;';
const WITDH_COL_C = 'width: 31%;';

// pagination
let paginationEnabled = false; // enable/disable pagination
let paginationRowLimit = 3; // assignment says 10, but we start with 5 first.
let pageNumbers = []; // will hold a list of numbers from 1 up to and including N for N pages
let selectedPage = 1; // wil hold the page number, anything between 1 up to and including N for N pages

// todo database
let todoList = []; // all todo entries are stored here

// todo metadata
let todoIndex = 0; // used as a primary key index for todoList (non-dependent of todoList.length)
let showIndexes = []; // only show todos filtered by pagination and select options
const STATUS_REFERENCE = {
    unchecked: 'Gjenstår', checked: 'Fullført', deleted: 'Slettet',
};

// visual options
let isShowDeleted = false;
let colorSwitchRow = true; // to make every other row a bit darker making them easier to distiguish

// for editing the text field in todo list
let editTodoIndexText = -1; // -1 = no selected todo-list
let editTodoText = '';

// for editing the responsible field in todo list
let editTodoIndexResponsible = -1; // -1 = no selected todo-list
let editTodoResponsible = '';

// filtering options for todo list
let filterByResponsible = '';
let filterByDoneDate = '';
