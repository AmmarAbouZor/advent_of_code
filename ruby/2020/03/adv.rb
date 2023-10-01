# frozen_string_literal: true

require '../../aoc_base'

# Year 2020 Day 03
class Aoc03 < AocBase
  def calc_slope(input, drow, dcol)
    # chomp to trim the end of line char '\n' from each line end
    lines = input.lines.map(&:chomp)
    row = col = 0
    count = 0
    col_count = lines[0].length
    while row < lines.length
      count += 1 if lines[row][col % col_count] == '#'
      row += drow
      col += dcol
    end
    count
  end

  def calc_all_slopes(input)
    slopes = [
      [1, 1],
      [1, 3],
      [1, 5],
      [1, 7],
      [2, 1]
    ]

    slopes
      .map { |slope| calc_slope(input, slope[0], slope[1]) }
      .inject(1) { |result, element| result * element }
  end

  def part_one
    calc_slope(@input, 1, 3)
  end

  def part_two
    calc_all_slopes(@input)
  end

  def do_tests
    assert_equal calc_slope(@test_input, 1, 3), 7
    assert_equal calc_all_slopes(@test_input), 336
    puts 'tests pass'
  end
end

if __FILE__ == $PROGRAM_NAME
  aoc = Aoc03.new
  aoc.run
end
