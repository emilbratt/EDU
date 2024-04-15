// test library
//
let testCountOK = 0;
let testCountFail = 0;
let testHTML = '';

function updateView() {
    // dummy function because all controllers calls this function
    return true;
}

function setup() {
    // define values for some variables before running tests
    barSelectedActivate = false;
    barSelectedIndex = 1;
    barSelectedHeight = 6;
}

function assertIdentical(alias, result, expect) {
    if (Array.isArray(result)) {
        result = result.toString();
        expect = expect.toString();
    } else if (typeof result === "object") {
        result = JSON.stringify(result);
        expect = JSON.stringify(expect);
    }

    if (result === expect) {
        testCountOK += 1;
        testHTML += `<span style="color: green;">OK</span>: ${alias}`;
    } else {
        testCountFail += 1;
        testHTML += `<span style="color: red;">Fail</span>: ${alias}`;
        testHTML += '<pre style="font-size: 1rem">Result\n<strong>' + result + '</strong> </pre>';
        testHTML += '<pre style="font-size: 1rem">Expected\n<strong>' + expect + '</strong> </pre>';
    }
    testHTML += '<hr>';
}

function printResults() {
    let summaryHTML = '';
    if (testCountFail > 0) {
        summaryHTML += '<h3>Test: <span style="color: red;">Failed</span></h3>';
    } else {
        summaryHTML += '<h3>Test: <span style="color: green;">Passed</span></h3>';
    }
    summaryHTML += `
        <strong>Passed</strong>: ${testCountOK}<br>
        <strong>Failed</strong>: ${testCountFail}<hr>
    `;
    document.body.innerHTML = summaryHTML + testHTML;
}


// test cases
//
function runTests() {
    // add testcases here and call assertIdentical()

    selectedSVGBar(barSelectedIndex);
    assertIdentical(
        alias='barSelectedActivate test 1',
        result=barSelectedActivate,
        expect=true
    );

    assertIdentical(
        alias='barArray test 1',
        result=barArray,
        expect=[1, 3, 5, 8, 10]
    );

    selectedBarChange();
    assertIdentical(
        alias='barArray test 2',
        result=barArray,
        expect=[1, 6, 5, 8, 10]
    );

    selectedBarRemove();
    assertIdentical(
        alias='barArray test 3',
        result=barArray,
        expect=[1, 5, 8, 10]
    );
    selectedBarRemove();
    assertIdentical(
        alias='barArray test 4',
        result=barArray,
        expect=[1, 8, 10]
    );

    
    numberInputAddBar();
    assertIdentical(
        alias='barArray test 5',
        result=barArray,
        expect=[1, 8, 10, 6]
    );

    assertIdentical(
        alias='simple dictionary test 1',
        result={'hello': 'world', 'hi': 'ha'},
        expect={'hello': 'world', 'hi': 'ho'}
    );
}
