# frozen_string_literal: true

require '../../aoc_base'

# Year 2020 Day 04
class Aoc04 < AocBase
  REQ_KEYS = %w[byr iyr eyr hgt hcl ecl pid].freeze
  def parse_input(input)
    pass_dict = input.split("\n\n")
    pass_dict.map { |pass| parse_pass(pass) }
  end

  def parse_pass(pass_text)
    pass_dict = {}
    pass_text.lines.each do |line|
      line.split(' ').each do |pair|
        parts = pair.split(':')
        pass_dict[parts[0]] = parts[1]
      end
    end

    pass_dict
  end

  def check_pass_has_required(pass_dict)
    REQ_KEYS.all? { |key| pass_dict.key?(key) }
  end

  def passes_with_required(input)
    pass_dicts = parse_input(input)
    pass_dicts.count { |pass_dict| check_pass_has_required(pass_dict) }
  end

  def part_one
    passes_with_required(@input)
  end

  def part_two
    nil
  end

  def do_tests
    assert_equal passes_with_required(@test_input), 2
    puts 'tests pass'
  end
end

if __FILE__ == $PROGRAM_NAME
  aoc = Aoc04.new
  aoc.run
end
