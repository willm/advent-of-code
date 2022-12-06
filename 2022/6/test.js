const test = require('node:test');
const assert = require('assert');
const fs = require('fs');

const isMarker = (chars) => {
  const list = chars.split('');
  return list.every(char => list.filter((c) => c === char).length === 1);
};

const getStartIndex = (data, uniqueCharCount, startIndex) => {
  startIndex = startIndex || uniqueCharCount;
  if (data.length < uniqueCharCount) {
    return null;
  }
  if (isMarker(data.slice(0, uniqueCharCount))) {
    return startIndex;
  }
  return getStartIndex(data.slice(1), uniqueCharCount, startIndex + 1);
};

const getStartOfPacket = (data) => getStartIndex(data, 4);
const getStartOfMessage = (data) => getStartIndex(data, 14);

const readFromFile = (file) => {
  return fs.readFileSync(file).toString('utf8');
};

test('identifying a marker', async () => {
  assert.equal(isMarker('abcd'), true);
  assert.equal(isMarker('efgh'), true);
  assert.equal(isMarker('wxyz'), true);
  assert.equal(isMarker('wxyw'), false);
  assert.equal(isMarker('aaaa'), false);
});


test('when the buffer starts with a marker', async () => {
  assert.equal(getStartOfPacket('abcd'), 4);
  assert.equal(getStartOfPacket('efgh'), 4);
  assert.equal(getStartOfPacket('wxyz'), 4);
});

test('test examples', async () => {
  assert.equal(getStartOfPacket('mjqjpqmgbljsphdztnvjfqwrcgsmlb'), 7);
  assert.equal(getStartOfPacket('bvwbjplbgvbhsrlpgdmjqwftvncz'), 5);
  assert.equal(getStartOfPacket('nppdvjthqldpwncqszvftbrmjlhg'), 6);
  assert.equal(getStartOfPacket('nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg'), 10);
  assert.equal(getStartOfPacket('zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw'), 11);
});

test('part 1', async () => {
  assert.equal(getStartOfPacket(readFromFile('./input.txt')), 1912);
});

test('part 2 examples', async () => {
  assert.equal(getStartIndex('abcdefghijklmn', 14), 14);
  assert.equal(getStartOfMessage('mjqjpqmgbljsphdztnvjfqwrcgsmlb'), 19);
  assert.equal(getStartOfMessage('bvwbjplbgvbhsrlpgdmjqwftvncz'), 23);
  assert.equal(getStartOfMessage('nppdvjthqldpwncqszvftbrmjlhg'), 23);
  assert.equal(getStartOfMessage('nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg'), 29);
  assert.equal(getStartOfMessage('zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw'), 26);
});

test('part 2', async () => {
  console.log(getStartOfMessage(readFromFile('./input.txt')));
});
