# frozen_string_literal: true

require '../../aoc_base'

# Year 2020 Day 01
class Aoc01 < AocBase
  TARGET = 2020

  def two_sum(input, target)
    sums = {}
    nums = input.lines.map { |line| line.to_i }
    nums.each do |num|
      sum_val = sums[num]
      return sum_val * num unless sum_val.nil?

      to_target = target - num
      sums[to_target] = num
    end
  end

  def part_one
    two_sum(@input, TARGET)
  end

  def part_two
    nil
  end

  def do_tests
    assert_equal two_sum(@test_input, TARGET), 514_579
    puts 'tests pass'
  end
end

if __FILE__ == $PROGRAM_NAME
  aoc = Aoc01.new
  aoc.run
end
