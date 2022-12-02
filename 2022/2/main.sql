-- PART 1

CREATE TABLE part1 (round varchar not null, score int);
INSERT INTO part1 (round, score)
VALUES ('A Y', 8),
('B Z', 9),
('C X', 7),
('A Z', 3),
('B X', 1),
('C Y', 2),
('A X', 4),
('B Y', 5),
('C Z', 6);

CREATE TABLE strategy_guide (round varchar);

SELECT SUM(score)
FROM strategy_guide sg
JOIN part1 p
ON p.round = sg.round;

-- PART 2

CREATE TABLE part2 (round varchar not null, score int);
INSERT INTO part2 (round, score)
VALUES ('A Y', 4),
('B Z', 9),
('C X', 2),
('A Z', 8),
('B X', 1),
('C Y', 6),
('A X', 3),
('B Y', 5),
('C Z', 7);

SELECT SUM(score)
FROM strategy_guide sg
JOIN part2 p
ON p.round = sg.round;
