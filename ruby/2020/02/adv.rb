# frozen_string_literal: true

require '../../aoc_base'

# Year 2020 Day 02
class Aoc02 < AocBase
  class Password
    def initialize(min, max, chr, text)
      @min = min
      @max = max
      @ch = chr
      @text = text
    end

    def self.from_string(string)
      parts = string.split(' ')
      range = parts[0].split('-')
      min = range[0].to_i
      max = range[1].to_i
      ch = parts[1][0]
      text = parts[2]

      new(min, max, ch, text)
    end

    def to_s
      "min: #{@min}, max: #{@max}, ch: #{@ch}, text: #{@text}"
    end

    def valid?
      count = @text.count(@ch)
      count >= @min && count <= @max
    end
  end

  def calc_valid_passes(input)
    input.lines
         .map { |line| Password.from_string(line) }
         .count { |pass| pass.valid? }
  end

  def part_one
    calc_valid_passes(@input)
  end

  def part_two
    nil
  end

  def do_tests
    assert_equal calc_valid_passes(@test_input), 2
    puts 'tests pass'
  end
end

if __FILE__ == $PROGRAM_NAME
  aoc = Aoc02.new
  aoc.run
end
