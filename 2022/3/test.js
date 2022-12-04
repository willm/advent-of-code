const test = require('node:test');
const assert = require('assert');
const fs = require('fs');

const part1 = (ruckSackInput) => {
  const ruckSacks = ruckSackInput.split('\n');
  return ruckSacks
    .map(sack => itemPriorities(inBothCompartments(sack)))
    .flat()
    .reduce(sum);
};

const part2 = (ruckSackInput) => {
  const ruckSacks = ruckSackInput.split('\n')
    .map(line => line.split(''))
    .filter(line => line.length);
  const groups = chunk(ruckSacks, 3);
  const groupBadges = groups.map(group => intersection(...group));
  const groupPriorities = groupBadges.map(badges => Math.min(...itemPriorities(badges)));

  return groupPriorities.flat().reduce(sum);
};

const getCompartments = (ruckSack) => {
  const comp1Limit = Math.floor(ruckSack.length / 2);
  const comp2Limit = (ruckSack.length / 2).toFixed(0);
  return [
    ruckSack.slice(0, comp1Limit).split(''),
    ruckSack.slice(comp2Limit, ruckSack.length).split('')
  ];
};

const inBothCompartments = (ruckSack) => {
  const [compartment1, compartment2] = getCompartments(ruckSack);
  assert.equal(compartment1.length, compartment2.length);
  return intersection(compartment1, compartment2);
};

const intersection = (a, ...b) => {
  return [...new Set(a)]
    .filter((x) => b.map(y => new Set(y)).every(z => z.has(x)));
};

const itemPriorities = (items) => {
  const priorities = 'abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ';
  return items.map(i => priorities.indexOf(i) + 1);
}

const sum = (total, next) => {
  total = typeof total === 'number' ? total : 0;
  return total + next;
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

test('finds items present in both compartments', async () => {
  const items = inBothCompartments('aa');
  assert.deepEqual(items, ['a']);
});

test('can find the priority of items', async () => {
  const items = itemPriorities(['a','b']);
  assert.deepEqual(items, [1, 2]);
});

test('splitting a rucksack into compartments', () => {
  const [compartment1, compartment2] = getCompartments('vJrwpWtwJgWrhcsFMMfFFhFp');
  assert.equal(compartment1.length, compartment2.length);
});

test('splitting a rucksack with odd items into compartments', () => {
  const [compartment1, compartment2] = getCompartments('nVHVFfggbQVmFFfhLpBpBTrLBCB');
  assert.equal(compartment1.length, 13);
  assert.deepEqual(compartment1, ['n', 'V', 'H', 'V', 'F', 'f', 'g', 'g', 'b', 'Q', 'V', 'm', 'F']);
  assert.equal(compartment2.length, 13);
  assert.deepEqual(compartment2, ['f','h','L','p','B','p', 'B','T','r','L','B','C','B']);
});

test('part1 test case', async () => {
  const input = readFromFile('./test-input.txt');
  assert.equal(part1(input), 157);
});

test('part 1', async () => {
  const input = readFromFile('./input.txt');
  console.log(part1(input));
});

test('chunking into groups', () => {
  const list = [1,2,3,4,5,6,7,8,9];
  const chunks = chunk(list, 3);
  assert.equal(chunks.length, 3);
});

test('part2 test case', () => {
  const input = readFromFile('./part2-test.txt');
  assert.equal(part2(input), 70);
});

test('part2', () => {
  const input = readFromFile('./input.txt');
  console.log(part2(input));
});
