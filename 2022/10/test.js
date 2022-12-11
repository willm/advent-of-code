const test = require('node:test');
const assert = require('assert');
const fs = require('fs');

const mapQueue = (input) => {
  const lines = input.split('\n');
  const opperations = [
    {
      name: 'noop',
      cycles: 1,
      action(_arg, x) {
        return x;
      }
    },
    {
      name: 'addx',
      cycles: 2,
      action (arg, x) {
        return x + arg;
      }
    }
  ];
  let cycle = 0;
  return lines.map((line) => {
    const [instruction, arg] = line.split(' ');
    const operation = opperations.find(o => o.name == instruction);
    if (!operation) {
      return null;
    }
    cycle = cycle + operation.cycles;
    return {
      cycle,
      action: operation.action.bind(null, Number(arg))
    };
  }).filter(x => x !== null);
};

const run = (queue, cycles)=> {
  let currentCycle = 0;
  let x = 1;
  while (currentCycle < cycles) {
    const op = queue.find(op => op.cycle === currentCycle);
    if (op) {
      x = op.action(x);
    }
    currentCycle += 1;
  }
  return x;
};

const part2 = (input) => {
  const queue = mapQueue(input);
  let currentCycle = 0;
  let x = 1;
  while (currentCycle <= 240) {
    const currentPixel = currentCycle % 40;
    if (currentPixel === 0) {
      process.stdout.write('\n');
    }
    const op = queue.find(op => op.cycle === currentCycle);
    if (op) {
      x = op.action(x);
    }
    if (currentPixel === x || (currentPixel + 1) === x || (currentPixel - 1) % 40 === x) {
      process.stdout.write('#');
    }
    else {
      process.stdout.write('.');
    }
    currentCycle += 1;
  }
}

const part1 = (input, end) => {
  const queue = mapQueue(input);
  const x = run(queue, end);
  return end * x;
};

const readFromFile = (file) => {
  return fs.readFileSync(file).toString('utf8');
};

test('mapping the queue of instructions', async () => {
  const input = `noop
addx 3
addx -5`;
  const queue = mapQueue(input);
  assert.equal(queue.length, 3);
  assert.equal(queue[0].action(0), 0);
  assert.equal(queue[0].cycle, 1);
  assert.equal(queue[1].action(0), 3);
  assert.equal(queue[1].cycle, 3);
  assert.equal(queue[2].action(0), -5);
  assert.equal(queue[2].cycle, 5);
});

test('test example', async () => {
  const input = `noop
addx 3
addx -5`;
  assert.equal(part1(input, 1), 1);
  assert.equal(part1(input, 2), 2);
  assert.equal(part1(input, 3), 3);
  assert.equal(part1(input, 4), 16);
});

test('test part1 sample', async () => {
  const input = readFromFile('./input-test.txt');
  assert.equal(part1(input, 20), 420);
  assert.equal(part1(input, 60), 1140);
  assert.equal(part1(input, 100), 1800);
  assert.equal(part1(input, 140), 2940);
  assert.equal(part1(input, 180), 2880);
  assert.equal(part1(input, 220), 3960);
});

test('part1', async () => {
  const input = readFromFile('./input.txt');
  const result = [20, 60, 100, 140, 180, 220].reduce((total, cycle) => {
    return total + part1(input, cycle);
  }, 0);
  assert.equal(result, 14340);
});

test('test part2', async () => {
  const input = readFromFile('./input-test.txt');
  part2(input);
});

test('part2', async () => {
  const input = readFromFile('./input.txt');
  part2(input);
});
