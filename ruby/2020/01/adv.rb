# frozen_string_literal: true

require '../../aoc_base'

# Year 2020 Day 01
class Aoc01 < AocBase
  TARGET = 2020

  def two_sum(nums, target)
    sums = {}
    nums.each do |num|
      sum_val = sums[num]
      return sum_val * num unless sum_val.nil?

      to_target = target - num
      sums[to_target] = num
    end
    nil
  end

  def two_sum_prod(input)
    nums = input.lines.map(&:to_i)
    two_sum(nums, TARGET)
  end

  def three_sum_prod(input)
    nums = input.lines.map(&:to_i)
    nums.each_with_index do |num, idx|
      target = TARGET - num
      two_sum_prd = two_sum(nums[idx + 1..], target)
      return two_sum_prd * num unless two_sum_prd.nil?
    end
    nil
  end

  def part_one
    two_sum_prod(@input)
  end

  def part_two
    three_sum_prod(@input)
  end

  def do_tests
    assert_equal two_sum_prod(@test_input), 514_579
    assert_equal three_sum_prod(@test_input), 241_861_950
    puts 'tests pass'
  end
end

if __FILE__ == $PROGRAM_NAME
  aoc = Aoc01.new
  aoc.run
end
