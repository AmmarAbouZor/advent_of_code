# frozen_string_literal: true

require '../../aoc_base'

# Year 2020 Day 11
class Aoc11 < AocBase
  DIRECTIONS = [[-1, -1], [-1, 0], [-1, 1], [0, -1], [0, 1], [1, -1], [1, 0], [1, 1]].freeze

  # Represents seats layout
  class Layout
    def initialize(cells)
      @cells = cells
    end

    def self.form_string(input)
      cells = input.split("\n").map(&:chars)
      Layout.new(cells)
    end

    def occupied_count
      @cells.flatten.count('#')
    end

    def count_occupied_surround(row, col)
      count = 0
      DIRECTIONS.each do |d_row, d_col|
        curr_row = row + d_row
        curr_col = col + d_col
        # cover is better for readability but has worse performance
        # if (0...@cells.length).cover?(curr_row)
        next unless curr_row >= 0 &&
                    curr_row < @cells.length &&
                    curr_col >= 0 &&
                    curr_col < @cells[0].length &&
                    @cells[curr_row][curr_col] == '#'

        count += 1
      end
      count
    end

    def apply_round_surround
      changed = false
      new_cells = []
      @cells.each_with_index do |row, r_idx|
        new_row = Array.new(@cells[0].length, '.')
        row.each_with_index do |ch, c_idx|
          case ch
          when 'L'
            if count_occupied_surround(r_idx, c_idx).zero?
              changed = true
              new_row[c_idx] = '#'
            else
              new_row[c_idx] = 'L'
            end
          when '#'
            if count_occupied_surround(r_idx, c_idx) >= 4
              changed = true
              new_row[c_idx] = 'L'
            else
              new_row[c_idx] = '#'
            end
          end
        end
        new_cells << new_row
      end
      @cells = new_cells
      changed
    end

    def count_occupied_vision(row, col)
      count = 0
      DIRECTIONS.each do |d_row, d_col|
        curr_row = row + d_row
        curr_col = col + d_col
        while curr_row >= 0 && curr_row < @cells.length && curr_col >= 0 && curr_col < @cells[0].length
          case @cells[curr_row][curr_col]
          when '#'
            count += 1
            break
          when 'L'
            break
          end
          curr_row += d_row
          curr_col += d_col
        end
      end
      count
    end

    def apply_round_vision
      changed = false
      new_cells = []
      @cells.each_with_index do |row, r_idx|
        new_row = Array.new(@cells[0].length, '.')
        row.each_with_index do |ch, c_idx|
          case ch
          when 'L'
            if count_occupied_vision(r_idx, c_idx).zero?
              changed = true
              new_row[c_idx] = '#'
            else
              new_row[c_idx] = 'L'
            end
          when '#'
            if count_occupied_vision(r_idx, c_idx) >= 5
              changed = true
              new_row[c_idx] = 'L'
            else
              new_row[c_idx] = '#'
            end
          end
        end
        new_cells << new_row
      end
      @cells = new_cells
      changed
    end
  end

  def get_surround_count(input)
    layout = Layout.form_string(input)
    while layout.apply_round_surround
    end

    layout.occupied_count
  end

  def get_vision_count(input)
    layout = Layout.form_string(input)
    while layout.apply_round_vision
    end

    layout.occupied_count
  end

  def part_one
    get_surround_count(@input)
  end

  def part_two
    get_vision_count(@input)
  end

  def do_tests
    assert_equal get_surround_count(@test_input), 37
    assert_equal get_vision_count(@test_input), 26
    puts 'tests pass'
  end
end

if __FILE__ == $PROGRAM_NAME
  aoc = Aoc11.new
  aoc.run
end
