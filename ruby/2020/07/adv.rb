# frozen_string_literal: true

require '../../aoc_base'
require 'set'

# Year 2020 Day 07
class Aoc07 < AocBase
  TARGET_NAME = 'shiny gold'
  # Represents an entry with name and count
  class BagEntry
    attr_reader :name, :count

    def initialize(name, count)
      @name = name
      @count = count
    end

    def to_s
      "name: '#{@name}', count: '#{@count}'"
    end

    def inspect
      to_s
    end
  end

  def parse_input(input)
    bags_map = {}
    input.lines.map(&:chomp).each do |line|
      name, rest = line.split(' bags contain ', 2)
      bags = rest.split(', ')
                 .reject { |chunk| chunk.start_with?('no') }
                 .map do |chunk|
        parts = chunk.split(' ')
        count = parts[0].to_i
        child_name = parts[1..2].join(' ')
        BagEntry.new(child_name, count)
      end

      bags_map[name] = bags
    end

    bags_map
  end

  def get_contain_count(input)
    bags_map = parse_input(input)

    containig_bags = Set.new
    containig_bags.add(TARGET_NAME)
    has_new_bags = true
    while has_new_bags
      has_new_bags = false
      bags_map.each do |name, children|
        next if containig_bags.include?(name)

        if children.any? { |child| containig_bags.include?(child.name) }
          containig_bags.add(name)
          has_new_bags = true
        end
      end
    end

    containig_bags.length - 1
  end

  def part_one
    get_contain_count(@input)
  end

  def part_two
    nil
  end

  def do_tests
    assert_equal get_contain_count(@test_input), 4
    puts 'tests pass'
  end
end

if __FILE__ == $PROGRAM_NAME
  aoc = Aoc07.new
  aoc.run
end
