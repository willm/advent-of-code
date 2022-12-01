require 'test/unit'

class Board
  attr_reader :lines
  def initialize(lines)
    @called_numbers = []
    @lines = lines
  end

  def call_number(num)
    @called_numbers.append(num)
    vertical_win = (0..@lines[0].length).reduce(false) {|win, i| 
      win || @lines.map{|x| x[i]}.all? {|x| @called_numbers.include?(x)}
    }
    horizontal_win = @lines.reduce(false) { |win, line| 
      win || line.all? { |x|
        @called_numbers.include?(x)
      }
    }
    horizontal_win || vertical_win
  end

  def score()
    @lines.map {|line| line.filter {
      |num| @called_numbers.include?(num) == false
    }}.flatten.reduce(0) {|sum, num| sum += num} * @called_numbers.last
  end
end

class BoardBuilder
  def initialize()
    @lines = []
  end

  def push_line(line)
    parsed = line.split(" ").map { |x| Integer(x) }
    @lines.append(parsed)
  end

  def build()
    Board.new(@lines)
  end
end

BingoGame = Struct.new(:numbers, :boards)

def get_numbers(line)
  line.split(',').map { |x| Integer(x) }
end

def get_boards(lines)
  lines.reduce([]) {|boards, line|
    if line.empty?
      boards.append(BoardBuilder.new)
    else
      boards.last.push_line(line)
    end
    boards
  }.map {|b| b.build }
end

def winning_board_scores(bingo_game)
  winners = []
  for num in bingo_game.numbers
    for board in bingo_game.boards
      if (board.call_number(num))
        winners.append(board.score)
        bingo_game.boards = bingo_game.boards.filter{|b| b != board}
      end
    end
  end
  winners
end

def game_from_file(path)
  game = nil
  File.open(path, 'r') do |file|
    lines = file.read.split("\n")
    game = BingoGame.new(
      get_numbers(lines[0]),
      get_boards(lines[1..])
    )
  end
  game
end

def main()
  game = game_from_file('./input.txt')
  winners = winning_board_scores(game)
  puts "winning board final score #{winners.first}"
  puts "loosing board final score #{winners.last}"
end

main()
