// model
//
const BAR_TOTAL_LIMIT = 10;
const BAR_MAX_HEIGHT = 10;
const BAR_MIN_HEIGHT = 1;

// stolper som blir produsert i svg bildet lagres her, ..hver verdi tilsvarer høyden
let barArray = [1, 3, 5, 8, 10];

// stolpe som man klikker på (vil inneholde et tall mellom 0-10)
let barSelectedIndex = 1;

// for at noe skal kunne skje med en stolpe, så må denne verdien være "true"
let barSelectedActivate = false;

// stolpe som er lagt til (eller valgt) får denne høyden
let barSelectedHeight = 5;
