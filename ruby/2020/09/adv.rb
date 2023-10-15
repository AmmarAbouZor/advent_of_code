# frozen_string_literal: true

require '../../aoc_base'
require 'set'

# Year 2020 Day 09
class Aoc09 < AocBase
  def two_sum?(target, nums)
    diff_set = Set.new
    nums.each do |num|
      return true if diff_set.include?(num)

      diff_set.add(target - num)
    end

    false
  end

  def get_first_invalid(depth, input)
    nums = input.lines.map(&:to_i)
    min_idx = 0
    top_idx = depth

    loop do
      return nums[top_idx] unless two_sum?(nums[top_idx], nums[min_idx...top_idx])

      min_idx += 1
      top_idx += 1
    end
  end

  def part_one
    get_first_invalid(25, @input)
  end

  def part_two
    nil
  end

  def do_tests
    assert_equal get_first_invalid(5, @test_input), 127
    puts 'tests pass'
  end
end

if __FILE__ == $PROGRAM_NAME
  aoc = Aoc09.new
  aoc.run
end
