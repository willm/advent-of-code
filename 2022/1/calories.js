const fs = require('fs');
const file = fs.readFileSync('./part1.txt').toString('utf8');

console.log(
  file.split("\n\n")
  .map(elf => elf.split("\n"))
  .map(elf => elf.reduce((count, snack) => count + Number(snack), 0))
  .sort()
  .reverse()
  .slice(0, 3)
  .reduce((acc, count) => acc + count, 0)
)
