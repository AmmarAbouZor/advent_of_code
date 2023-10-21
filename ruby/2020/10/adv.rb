# frozen_string_literal: true

require '../../aoc_base'

# Year 2020 Day 10
class Aoc10 < AocBase
  def calc_diff_prod(input)
    nums = input.lines.map(&:to_i)
    nums.sort!

    nums << nums[-1] + 3

    diff1 = 0
    diff3 = 0
    current = 0

    nums.each do |num|
      case num - current
      when 1
        diff1 += 1
      when 3
        diff3 += 1
      end

      current = num
    end

    diff1 * diff3
  end

  def part_one
    calc_diff_prod(@input)
  end

  def part_two
    nil
  end

  def do_tests
    assert_equal calc_diff_prod(@test_input), 22 * 10
    puts 'tests pass'
  end
end

if __FILE__ == $PROGRAM_NAME
  aoc = Aoc10.new
  aoc.run
end
