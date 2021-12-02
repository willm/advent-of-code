import unittest

class Tests(unittest.TestCase):
    def test_example(self):
        input = """
forward 5
down 5
forward 8
up 3
down 8
forward 2
"""
        coordinates = perform_journey(input)
        self.assertEqual(coordinates["horizontal"], 15)
        self.assertEqual(coordinates["depth"], 60)

def perform_journey(input):
    commands = list(map(split_words, filter(not_empty_string, input.split('\n'))))
    directions = {
        "forward": forward,
        "down": down,
        "up": up
    }

    current_position = {"horizontal": 0, "depth": 0, "aim": 0}
    for direction, step in commands:
        directions[direction](current_position, int(step))
    return current_position


def forward(coordinates, step):
    coordinates["horizontal"] += step
    coordinates["depth"] += coordinates["aim"] * step

def down(coordinates, step):
    coordinates["aim"] += step

def up(coordinates, step):
    coordinates["aim"] -= step

def split_words(s):
    return s.split(' ')

def not_empty_string(s):
    return s != ''

if __name__ == "__main__":
    with open('./input.txt', 'r') as file:
        final_postition = perform_journey(file.read())
        print(final_postition["horizontal"] * final_postition["depth"])

