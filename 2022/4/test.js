const test = require('node:test');
const assert = require('assert');
const fs = require('fs');

const part1 = (input) => {
  return parseRanges(input)
    .reduce((overlapingPairs, pair) => {
      return totallyOverlaps(pair) ? overlapingPairs + 1 : overlapingPairs
    }, 0);
};

const part2 = (input) => {
  return parseRanges(input)
    .reduce((overlapingPairs, pair) => {
      return partiallyOverlaps(pair) ? overlapingPairs + 1 : overlapingPairs
    }, 0);
};

const readFromFile = (file) => {
  return fs.readFileSync(file).toString('utf8');
};

const parseRanges = (input) => {
  const rangePair = /(\d+)\-(\d+),(\d+)\-(\d+)/;
  return input.split('\n').filter(line => line).map(line => {
    const match = line.match(rangePair);
    return [
      [Number(match[1]), Number(match[2])],
      [Number(match[3]), Number(match[4])]
    ]
  });
};

const totallyOverlaps = (pairs) => {
  const jobs1 = jobsInRange(pairs[0]);
  const jobs2 = jobsInRange(pairs[1]);
  return jobs1.every(job => jobs2.includes(job)) ||
    jobs2.every(job => jobs1.includes(job));
};
const jobsInRange = (range) => {
  return Array.from(Array(range[1] - range[0] + 1).fill(range[0]), (x, i) => x + i);
}

const partiallyOverlaps = (pairs) => {
  const jobs1 = jobsInRange(pairs[0]);
  const jobs2 = jobsInRange(pairs[1]);
  return jobs1
    .some(job => jobs2.includes(job));
};

test('can parse ranges', async () => {
  const ranges = parseRanges(`2-4,6-8
2-3,4-5
2-6,4-8`);
  assert.equal(ranges.length, 3);
  assert.deepEqual(ranges[0], [[2, 4],[6, 8]]);
  assert.deepEqual(ranges[1], [[2, 3],[4, 5]]);
  assert.deepEqual(ranges[2], [[2, 6],[4, 8]]);
});

[
  [`2-4,6-8`, [ [[2, 4],[6, 8]] ]],
  [`56-77,55-82`, [ [[56, 77],[55, 82]] ]]
].forEach(([input, expected]) => {
  test('can parse range', async () => {
    const ranges = parseRanges(input);
    assert.deepEqual(ranges, expected);
  });
});

test('total overlaps', async () => {
  assert.equal(totallyOverlaps([[0,5], [0, 2]]), true);
  assert.equal(totallyOverlaps([[2,8], [3, 7]]), true);
  assert.equal(totallyOverlaps([[6,6], [4, 6]]), true);
});

test('partial overlaps', async () => {
  assert.equal(partiallyOverlaps([[0,5], [0, 2]]), true);
  assert.equal(partiallyOverlaps([[2,8], [3, 7]]), true);
  assert.equal(partiallyOverlaps([[6,6], [4, 6]]), true);
  assert.equal(partiallyOverlaps([[2,6], [4, 8]]), true);
  assert.equal(partiallyOverlaps([[2,4], [6, 8]]), false);
});

test('jobs in range', async () => {
  const jobs = jobsInRange([2,5]);
  assert.deepEqual(jobs, [2,3,4,5]);
});

test('test part1', () => {
  const ranges = readFromFile('./test-input.txt');
  assert.equal(part1(ranges), 2);
});

test('part1', () => {
  const ranges = readFromFile('./input.txt');
  assert.equal(part1(ranges), 498);
});

test('test part2', () => {
  const ranges = readFromFile('./test-input.txt');
  assert.equal(part2(ranges), 4);
});

test('part2', () => {
  const ranges = readFromFile('./input.txt');
  assert.equal(part2(ranges), 859);
});
