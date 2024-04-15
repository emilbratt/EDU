// TEST fixText()
QUnit.test("fix - 'terje'", function (assert) {
    const actual = fixText('terje');
    const expected = 'Terje';
    assert.equal(actual, expected);
});

QUnit.test("fix - 'terje '", function (assert) {
    const actual = fixText('terje ');
    const expected = 'Terje';
    assert.equal(actual, expected);
});

QUnit.test("fix - ' terje'", function (assert) {
    const actual = fixText(' terje');
    const expected = 'Terje';
    assert.equal(actual, expected);
});

QUnit.test("fix - ' terje '", function (assert) {
    const actual = fixText(' terje ');
    const expected = 'Terje';
    assert.equal(actual, expected);
});

QUnit.test("fix - ' terje '", function (assert) {
    const actual = fixText(' tErjE');
    const expected = 'Terje';
    assert.equal(actual, expected);
});

QUnit.test("fix - ' terje '", function (assert) {
    const actual = fixText(' tErJe ');
    const expected = 'Terje';
    assert.equal(actual, expected);
});


// TEST isEmail()
QUnit.test('isEmail', function (assert) {
    const actual = isEmail('hello.world.gmail.com');
    const expected = false;
    assert.equal(actual, expected);
});

QUnit.test('isEmail', function (assert) {
    const actual = isEmail('hello.world.gmail@com');
    const expected = false;
    assert.equal(actual, expected);
});

QUnit.test('isEmail', function (assert) {
    const actual = isEmail('hello.world@gmail.com');
    const expected = true;
    assert.equal(actual, expected);
});
