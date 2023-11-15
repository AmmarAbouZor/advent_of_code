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
                .filter_map { |id| id.to_i if id.match?(/\d+/) }
                .map { |bus_id| [min_diff(bus_id, target), bus_id] }
                .min_by { |diff, _id| diff }

    result[0] * result[1]
  end

  # @param [String] input
  def calc_earliest_match(input)
    buses = input.lines(chomp: true)[1]
    records = buses.split(',')
                   .each_with_index
                   .filter_map { |bus, idx| [idx, bus.to_i] if bus.match?(/\d+/) }

    lcm = 1
    time = 0
    (0...(records.length - 1)).each do |i|
      bus = records[i + 1][1]
      idx = records[i + 1][0]
      lcm *= records[i][1]
      time += lcm while (time + idx) % bus != 0
    end

    time
  end

  def part_one
    calc_earliest(@input)
  end

  def part_two
    calc_earliest_match(@input)
  end

  def do_tests
    assert_equal calc_earliest(@test_input), 295
    assert_equal calc_earliest_match(@test_input), 1_068_781
    puts 'tests pass'
  end
end

if __FILE__ == $PROGRAM_NAME
  aoc = Aoc13.new
  aoc.run
end
