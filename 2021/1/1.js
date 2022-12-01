const fs = require('fs');
const assert = require('assert');

const main = async () => {
  const input = fs.readFileSync('./input.txt')
    .toString('utf8')
    .split('\n')
    .map(Number);

  console.log(numberOfIncreases(input));
};

const numberOfIncreases = (input) => {
  return input.reduce((increases, depth, index) => {
    if (index === 0) {
      return 0;
    }
    const previousWindow = sum(input.slice(index - 1, index + 2));
    const currentWindow = sum(input.slice(index, index + 3));
    if (currentWindow > previousWindow) {
      return increases + 1;
    }
    return increases;
  }, 0);
};

const sum = (values) => values.reduce((sum, i) => sum + i, 0);

main();

// tests

assert.equal(numberOfIncreases([199, 200, 208, 210]), 1);
assert.equal(numberOfIncreases([
  607,
  618,
  618,
  617,
  647,
  716,
  769,
  792,
]), 5);
