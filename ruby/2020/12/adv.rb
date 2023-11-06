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

  class StateWaypoint
    def initialize
      @pos_x = 0
      @pos_y = 0
      @way_x = 10
      @way_y = 1
    end

    def manhatten_distance
      @pos_x.abs + @pos_y.abs
    end

    def rotate(angle_degree)
      angle_rad = angle_degree * Math::PI / 180
      sin = Math.sin(angle_rad)
      cos = Math.cos(angle_rad)
      x = @way_x * cos - @way_y * sin
      y = @way_x * sin + @way_y * cos
      @way_x = x.round
      @way_y = y.round
    end

    def move_forward(num)
      @pos_x += @way_x * num
      @pos_y += @way_y * num
    end

    def apply_inst(inst)
      case inst.command
      when 'N'
        @way_y += inst.num
      when 'S'
        @way_y -= inst.num
      when 'E'
        @way_x += inst.num
      when 'W'
        @way_x -= inst.num
      when 'L'
        rotate(inst.num)
      when 'R'
        rotate(-inst.num)
      when 'F'
        move_forward(inst.num)
      else
        raise "Invalid command #{inst.command}"
      end
    end
  end

  def calc_distance(input, state)
    input
      .lines
      .map { |line| Instruction.from_string(line) }
      .each do |inst|
        state.apply_inst(inst)
      end

    state.manhatten_distance
  end

  def calc_direction_distance(input)
    state = StateDirection.new
    calc_distance(input, state)
  end

  def calc_waypoint_distance(input)
    state = StateWaypoint.new
    calc_distance(input, state)
  end

  def part_one
    calc_direction_distance(@input)
  end

  def part_two
    calc_waypoint_distance(@input)
  end

  def do_tests
    assert_equal calc_direction_distance(@test_input), 25
    assert_equal calc_waypoint_distance(@test_input), 286
    puts 'tests pass'
  end
end

if __FILE__ == $PROGRAM_NAME
  aoc = Aoc12.new
  aoc.run
end
