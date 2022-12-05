const test = require('node:test');
const assert = require('assert');
const fs = require('fs');

const part1 = (input) => {
  const {stacks, moves} = parse(input);
  moves.forEach(move => {
    Array.from({length: move[0]}).forEach(_ => {
      const crate = stacks[move[1] - 1].pop();
      stacks[move[2] - 1].push(crate);
    })
  });
  return stacks.reduce((topCrates, stack) => {
    return topCrates + stack.reverse()[0];
  }, '')
}

const part2 = (input) => {

  const {stacks, moves} = parse(input);
  moves.forEach(move => {
    const src = stacks[move[1] - 1];
    const dest = stacks[move[2] - 1];
    const blocks = src.slice(src.length - move[0]);
    stacks[move[1] - 1] = src.slice(0, src.length - move[0]);
    stacks[move[2] - 1] = dest.concat(blocks);
  });
  return stacks.reduce((topCrates, stack) => {
    return topCrates + stack.reverse()[0];
  }, '')
};

const parse = (input) => {
  const [stacksInput, moveInput] = input.split('\n\n');
  const stacks = stacksInput.match(/(\d+)/g).map(_ => []);
  const lines = stacksInput.split('\n');
  lines
    .slice(0, lines.length - 1)
    .reverse()
    .map(line => chunk(line.split(''), 4))
    .forEach((line) => {
      line.forEach((step, index) => {
        if(step[1] !== ' ') {
          stacks[index].push(step[1]);
        }
      });
    });

  const moves = moveInput.split('\n')
    .filter(line => line !== '')
    .map(line => line.match(/\d+/g).map(Number));
  return {stacks, moves};
};

const chunk = (list, chunkSize) => {
  const chunks = [];
  for (let i = 0; i < list.length; i += chunkSize) {
      chunks.push(list.slice(i, i + chunkSize));
  }
  return chunks;
};

const readFromFile = (file) => {
  return fs.readFileSync(file).toString('utf8');
};

test('parsing stacks', async () => {
  const setup = parse(`    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2`);

  assert.equal(setup.stacks.length, 3);
  assert.deepEqual(setup.stacks[0], ['Z', 'N']);
  assert.deepEqual(setup.stacks[1], ['M', 'C', 'D']);
  assert.deepEqual(setup.stacks[2], ['P']);
  assert.deepEqual(setup.moves, [
    [1, 2, 1],
    [3, 1, 3],
    [2, 2, 1],
    [1, 1, 2],
  ]);
});

test('test part 1', async () => {
  const input = readFromFile('./test-input.txt');
  assert.equal(part1(input), 'CMZ');
});

test('part 1', async () => {
  const input = readFromFile('./input.txt');
  console.log(part1(input));
});

test('test part 2', async () => {
  const input = readFromFile('./test-input.txt');
  assert.equal(part2(input), 'MCD');
});

test('part 2', async () => {
  const input = readFromFile('./input.txt');
  console.log(part2(input));
});
