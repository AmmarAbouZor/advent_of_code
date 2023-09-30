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

    def valid_1?
      count = @text.count(@ch)
      count >= @min && count <= @max
    end

    def valid_2?
      ch1 = @text[@min - 1]
      ch2 = @text[@max - 1]
      ch1 != ch2 && (ch1 == @ch || ch2 == @ch)
    end
  end

  def calc_valid_passes1(input)
    input.lines
         .map { |line| Password.from_string(line) }
         .count(&:valid_1?)
  end

  def calc_valid_passes2(input)
    input.lines
         .map { |line| Password.from_string(line) }
         .count(&:valid_2?)
  end

  def part_one
    calc_valid_passes1(@input)
  end

  def part_two
    calc_valid_passes2(@input)
  end

  def do_tests
    assert_equal calc_valid_passes1(@test_input), 2
    assert_equal calc_valid_passes2(@test_input), 1
    puts 'tests pass'
  end
end

if __FILE__ == $PROGRAM_NAME
  aoc = Aoc02.new
  aoc.run
end
