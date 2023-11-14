# frozen_string_literal: true

require '../../aoc_base'

# Year 2020 Day 13
class Aoc13 < AocBase
  # @param [Integer] bus_id
  # @param [Integer] target
  def min_diff(bus_id, target)
    num_before = target / bus_id
    first_arrive = bus_id * (num_before + 1)
    first_arrive - target
  end

  # @param [String] input
  def calc_earliest(input)
    lines = input.lines(chomp: true)
    target = lines[0]
    ids = lines[1]
    target = target.to_i

    result = ids.split(',')
                .map { |id| id.to_i if id.match?(/\d+/) }
                .compact
                .map { |bus_id| [min_diff(bus_id, target), bus_id] }
                .min_by { |diff, _id| diff }

    result[0] * result[1]
  end

  def part_one
    calc_earliest(@input)
  end

  def part_two
    nil
  end

  def do_tests
    assert_equal calc_earliest(@test_input), 295
    puts 'tests pass'
  end
end

if __FILE__ == $PROGRAM_NAME
  aoc = Aoc13.new
  aoc.run
end
