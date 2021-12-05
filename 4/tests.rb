require './main.rb'

class Tests < Test::Unit::TestCase
  def test_parsing
    game = game_from_file('./input-test.txt')
    assert_equal(
      game.numbers,
      [
        7, 4, 9, 5, 11,
        17, 23, 2, 0, 14,
        21, 24, 10, 16,
        13, 6, 15, 25,
        12, 22, 18, 20,
        8, 19, 3, 26, 1
      ]
    )
    assert_equal(game.boards.length, 3)
    assert_equal(game.boards[0].lines.length, 5)
    assert_equal(game.boards[0].lines[0], [22, 13, 17, 11, 0])
    assert_equal(game.boards[1].lines[0], [3, 15, 0, 2, 22])
  end

  def test_winning_board_horizontal()
    board = Board.new([
      [14, 21, 17, 24, 4],
      [10, 16, 15,  9, 19],
      [18,  8, 23, 26, 20],
      [22, 11, 13,  6,  5],
      [ 2,  0, 12,  3,  7]
    ])
    assert_equal(board.call_number(14), false)
    assert_equal(board.call_number(21), false)
    assert_equal(board.call_number(17), false)
    assert_equal(board.call_number(24), false)
    assert_equal(board.call_number(4), true)
  end

  def test_winning_board_vertical()
    board = Board.new([
      [14, 21, 17, 24, 4],
      [10, 16, 15,  9, 19],
      [18,  8, 23, 26, 20],
      [22, 11, 13,  6,  5],
      [ 2,  0, 12,  3,  7]
    ])
    assert_equal(board.call_number(14), false)
    assert_equal(board.call_number(10), false)
    assert_equal(board.call_number(18), false)
    assert_equal(board.call_number(22), false)
    assert_equal(board.call_number(2), true)
    assert_equal(board.score, 518)
  end

  def test_winning_score
    game = game_from_file('./input-test.txt')
    scores = winning_board_scores(game)
    assert_equal(scores.first, 4512)
    puts scores
    assert_equal(scores.last, 1924)
  end
end
