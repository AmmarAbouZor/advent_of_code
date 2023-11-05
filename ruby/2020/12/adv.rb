# frozen_string_literal: true

require '../../aoc_base'

# Year 2020 Day 12
class Aoc12 < AocBase
  # Represent a ship instruction
  class Instruction
    attr_reader :command, :num

    def initialize(command, num)
      @command = command
      @num = num
    end

    def self.from_string(text)
      command = text[0]
      num = text[1..].to_i
      Instruction.new(command, num)
    end

    def to_s
      "Command: #{@command}, Num: #{@num}"
    end

    def inspect
      to_s
    end
  end

  # The ship state using direction
  class StateDirection
    DIR_LEFT = { 'E' => 'N', 'N' => 'W', 'W' => 'S', 'S' => 'E' }.freeze
    DIR_RIGHT = { 'E' => 'S', 'S' => 'W', 'W' => 'N', 'N' => 'E' }.freeze
    def initialize
      @pos_x = 0
      @pos_y = 0
      @direction = 'E'
    end

    def manhatten_distance
      @pos_x.abs + @pos_y.abs
    end

    def turn_left
      @direction = DIR_LEFT[@direction]
    end

    def turn_right
      @direction = DIR_RIGHT[@direction]
    end

    def apply_inst(inst)
      case inst.command
      when 'N'
        @pos_y += inst.num
      when 'S'
        @pos_y -= inst.num
      when 'E'
        @pos_x += inst.num
      when 'W'
        @pos_x -= inst.num
      when 'L'
        count = inst.num / 90
        count.times do |_|
          turn_left
        end
      when 'R'
        count = inst.num / 90
        count.times do |_|
          turn_right
        end
      when 'F'
        new_inst = Instruction.new(@direction, inst.num)
        apply_inst(new_inst)
      else
        raise "Invalid command #{inst.command}"
      end
    end
  end

  def calc_direction_distance(input)
    state = StateDirection.new
    input
      .lines
      .map { |line| Instruction.from_string(line) }
      .each do |inst|
        state.apply_inst(inst)
      end

    state.manhatten_distance
  end

  def part_one
    calc_direction_distance(@input)
  end

  def part_two
    nil
  end

  def do_tests
    assert_equal calc_direction_distance(@test_input), 25
    puts 'tests pass'
  end
end

if __FILE__ == $PROGRAM_NAME
  aoc = Aoc12.new
  aoc.run
end
