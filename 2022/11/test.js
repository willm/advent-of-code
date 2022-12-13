const test = require('node:test');
const assert = require('assert');
const fs = require('fs');

const parseMonkeys = (input) => {
  const monkeys = input.split('\n\n');
  return monkeys
    .map(parseMonkey);
};

const parseMonkey = (input) => {
  const lines = input.split('\n');
  const [_, itemsLine, operationLine, ...testLines] = lines;

  const startingItems = itemsLine.match(/(\d+)+/gm).map(Number);

  const {operator, digit} = operationLine
    .match(/old (?<operator>[\+\-\*\\]) (?<digit>\d+|old)/).groups;
  const operators = {
    '*': (x, y) => x * y,
    '+': (x, y) => x + y,
    '-': (x, y) => x - y,
    '/': (x, y) => x / y,
  };

  const [divisibleBy, trueMonkey, falseMonkey] = testLines
    .join('\n')
    .match(/(\d+)/gm)
    .map(Number);

  return {
    startingItems,
    operation:(old) => {
      let actualDigit = digit === 'old' ? old : Number(digit);
      return operators[operator](old, actualDigit)
    },
    test: (x) => x % Number(divisibleBy) === 0 ?
      [x, Number(trueMonkey)] :
      [x, Number(falseMonkey)],
    divisibleBy
  };
};

const monkeyInTheMiddle = (
  input,
  worryFn = (worryLevel) => Math.floor(worryLevel / 3)
) => {
  const monkeys = parseMonkeys(input);
  const max = monkeys.reduce((highest, m) => highest * m.divisibleBy, 1);
  const inspectedItems = Array.from({length: monkeys.length}, () => 0);
  return {
    runRound: () => {
      monkeys.forEach((monkey, i) => {
        monkey.startingItems.forEach((item) => {
          inspectedItems[i] += 1;
          let worryLevel = monkey.operation(item) % max;
          worryLevel = worryFn(worryLevel);
          const [_, nextMonkey] = monkey.test(worryLevel);
          monkeys[nextMonkey].startingItems.push(worryLevel);
          monkey.startingItems = monkey.startingItems.slice(1);
        });
      })
      return monkeys;
    },
    timesInspected: () => inspectedItems,
    monkeyBusiness: () => {
      inspectedItems.sort();
      const topMonkeys = inspectedItems.sort((a, b) => b - a).slice(0, 2);
      return topMonkeys[0] * topMonkeys[1];
    }
  }
};

const constantWorryLevel = (worryLevel) => {
  const after = Math.floor(worryLevel)
  return after;
};

const readFromFile = (file) => {
  return fs.readFileSync(file).toString('utf8');
};

test('parsing a monkey', async () => {
  const input = `Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3`;

  const monkey = parseMonkey(input);

  assert.deepEqual(monkey.startingItems, [79, 98]);
  assert.equal(monkey.operation(3), 57);
  assert.deepEqual(monkey.test(23), [23, 2]);
  assert.deepEqual(monkey.test(21), [21, 3]);
});

test('parsing a monkey with an operation using item value', async () => {
  const input = `Monkey 0:
  Starting items: 79, 98
  Operation: new = old * old
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3`;

  const monkey = parseMonkey(input);

  assert.deepEqual(monkey.startingItems, [79, 98]);
  assert.equal(monkey.operation(3), 9);
  assert.deepEqual(monkey.test(23), [23, 2]);
  assert.deepEqual(monkey.test(21), [21, 3]);
});

const input = `Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1`;

test('parsing many monkeys', async () => {
  const monkeys = parseMonkeys(input);

  assert.equal(monkeys.length, 4);
});

test('round', async () => {
  const game = monkeyInTheMiddle(input);
  const result = game.runRound();
  assert.deepEqual(result[0].startingItems, [20, 23, 27, 26]);
  assert.deepEqual(game.timesInspected(), [2, 4, 3, 5]);
});

test('test part 1', async () => {
  const game = monkeyInTheMiddle(input);

  Array.from({length: 20}).forEach(_ => game.runRound());

  assert.deepEqual(game.timesInspected(), [ 101, 95, 7, 105 ]);
  assert.deepEqual(game.monkeyBusiness(), 10605);
});

test('part 1', async () => {
  const game = monkeyInTheMiddle(readFromFile('./input.txt'));

  Array.from({length: 20}).forEach(_ => game.runRound());

  assert.equal(game.monkeyBusiness(), 54253);
});

test('test part 2 1 round', async () => {
  const game = monkeyInTheMiddle(input, constantWorryLevel);

  game.runRound(constantWorryLevel);

  assert.deepEqual(game.timesInspected(), [2,4,3,6]);
});

test('test part 2 20 rounds', async () => {
  const game = monkeyInTheMiddle(input, constantWorryLevel);

  Array.from({length: 20})
    .forEach(_ => game.runRound());

  assert.deepEqual(game.timesInspected(), [99,97,8,103]);
});

test('part 2', async () => {
  const game = monkeyInTheMiddle(
    readFromFile('./input.txt'),
    constantWorryLevel
  );

  Array.from({length: 10000})
    .forEach(_ => game.runRound());

  //14397120144
  console.log(game.monkeyBusiness());
});
