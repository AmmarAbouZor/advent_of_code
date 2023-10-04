# frozen_string_literal: true

require '../../aoc_base'

# Year 2020 Day 06
class Aoc06 < AocBase
  # Represents a Seat
  class Seat
    def initialize(lines)
      @lines = lines
    end

    def self.from_string(input)
      lines = input.lines.map(&:chomp).map(&:chars)
      Seat.new(lines)
    end

    def to_s
      @lines.to_s
    end

    def calc_unique_count
      @lines.flatten.uniq.count
    end

    def calc_intersect_count
      @lines.reduce(:&).count
    end
  end

  def calc_unique_sum(input)
    input.split(/\n\n/)
         .map { |part| Seat.from_string(part) }
         .map(&:calc_unique_count)
         .sum
  end

  def calc_intersect_sum(input)
    input.split(/\n\n/)
         .map { |part| Seat.from_string(part) }
         .map(&:calc_intersect_count)
         .sum
  end

  def part_one
    calc_unique_sum(@input)
  end

  def part_two
    calc_intersect_sum(@input)
  end

  def do_tests
    assert_equal calc_unique_sum(@test_input), 11
    assert_equal calc_intersect_sum(@test_input), 6
    puts 'tests pass'
  end
end

if __FILE__ == $PROGRAM_NAME
  aoc = Aoc06.new
  aoc.run
end
