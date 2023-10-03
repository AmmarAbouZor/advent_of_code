# frozen_string_literal: true

require '../../aoc_base'

# Year 2020 Day 05
class Aoc05 < AocBase
  # Modelling a seat to provide the id
  class Seat
    def initialize(rows, cols)
      @rows = rows
      @cols = cols
    end

    def self.from_string(line)
      rows = line[..6]
      cols = line[-3..]

      Seat.new(rows, cols)
    end

    def to_s
      "rows: #{@rows}, cols: #{@cols}"
    end

    def calc_id
      row = solve_row
      col = solve_col
      row * 8 + col
    end

    def solve_row
      min = 0
      max = 127
      @rows.chars.each do |char|
        case char
        when 'B'
          min = (min + max) / 2 + 1
        when 'F'
          max = (min + max) / 2
        else
          raise "Rows: Invalid char #{char}"
        end
      end

      min
    end

    def solve_col
      min = 0
      max = 7

      @cols.chars.each do |char|
        case char
        when 'R'
          min = (min + max) / 2 + 1
        when 'L'
          max = (min + max) / 2
        else
          raise "Cols: Invalid char '#{char}'"
        end
      end

      min
    end
  end

  def part_one
    @input
      .split(/\n/)
      .map { |line| Seat.from_string(line).calc_id }
      .max
  end

  def part_two
    all_ids = @input
              .split(/\n/)
              .map { |line| Seat.from_string(line).calc_id }
              .sort

    last_id = all_ids[0]
    all_ids[1..].each do |id|
      return id - 1 if id - last_id != 1

      last_id = id
    end
  end

  def do_tests
    input = { 'FBFBBFFRLR' => 357, 'BFFFBBFRRR' => 567, 'FFFBBBFRRR' => 119, 'BBFFBBFRLL' => 820 }
    input.each do |key, value|
      seat = Seat.from_string(key)
      assert_equal seat.calc_id, value
    end
    puts 'tests pass'
  end
end

if __FILE__ == $PROGRAM_NAME
  aoc = Aoc05.new
  aoc.run
end
