# frozen_string_literal: true

require '../../aoc_base'
require 'set'

# Year 2020 Day 08
class Aoc08 < AocBase
  class Instruction
    def initialize(typ, num)
      @typ = typ
      @num = num
    end

    def self.from_string(txt)
      typ, num = txt.split(' ')
      typ = case typ
            when 'acc' then :acc
            when 'jmp' then :jmp
            when 'nop' then :nop
            else raise "Invalid input '#{typ}'"
            end
      Instruction.new(typ, num.to_i)
    end

    def inspect
      to_s
    end

    def to_s
      "type: #{@typ}, num: #{@num}"
    end

    # returns a tuple of (idx_diff, acc_diff)
    def apply
      case @typ
      when :acc then [1, @num]
      when :jmp then [@num, 0]
      when :nop then [1, 0]
      end
    end

    def swap_if_possible
      case @typ
      when :acc then nil
      when :jmp then Instruction.new(:nop, @num)
      when :nop then Instruction.new(:jmp, @num)
      end
    end
  end

  def parse_input(input)
    input.lines.map(&:chomp).map { |line| Instruction.from_string(line) }
  end

  def calc_acc_by_infinite(input)
    insts = parse_input(input)
    idx = 0
    acc = 0
    visited_idx = Set.new
    loop do
      return acc if visited_idx.add?(idx).nil?

      idx_diff, acc_diff = insts[idx].apply
      acc += acc_diff
      idx += idx_diff
    end
  end

  def try_solve(idx, acc, visited_set, insts)
    while idx < insts.length
      return nil if visited_set.add?(idx).nil?

      idx_diff, acc_diff = insts[idx].apply
      acc += acc_diff
      idx += idx_diff
    end
    acc
  end

  def calc_acc_fixed(input)
    insts = parse_input(input)
    idx = 0
    acc = 0
    visited_idx = Set.new
    loop do
      swapd = insts[idx].swap_if_possible

      unless swapd.nil?
        idx_diff, acc_diff = swapd.apply
        t_acc = acc + acc_diff
        t_idx = idx + idx_diff
        val = try_solve(t_idx, t_acc, visited_idx.clone, insts)
        return val unless val.nil?
      end

      raise "Can't be duplicated in visited_set" if visited_idx.add?(idx).nil?

      idx_diff, acc_diff = insts[idx].apply
      acc += acc_diff
      idx += idx_diff
    end
  end

  def part_one
    calc_acc_by_infinite(@input)
  end

  def part_two
    calc_acc_fixed(@input)
  end

  def do_tests
    assert_equal calc_acc_by_infinite(@test_input), 5
    assert_equal calc_acc_fixed(@test_input), 8
    puts 'tests pass'
  end
end

if __FILE__ == $PROGRAM_NAME
  aoc = Aoc08.new
  aoc.run
end
