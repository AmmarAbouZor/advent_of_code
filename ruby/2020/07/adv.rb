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

    containing_bags = Set.new
    containing_bags.add(TARGET_NAME)
    has_new_bags = true
    while has_new_bags
      has_new_bags = false
      bags_map.each do |name, children|
        next if containing_bags.include?(name)

        if children.any? { |child| containing_bags.include?(child.name) }
          containing_bags.add(name)
          has_new_bags = true
        end
      end
    end

    containing_bags.length - 1
  end

  def get_shiny_cost(input)
    bags_map = parse_input(input)
    get_child_count(TARGET_NAME, bags_map) - 1
  end

  def get_child_count(bag_name, bags_map)
    bags_map[bag_name].inject(1) { |acc, child| acc + get_child_count(child.name, bags_map) * child.count }
  end

  def part_one
    get_contain_count(@input)
  end

  def part_two
    get_shiny_cost(@input)
  end

  def do_tests
    assert_equal get_contain_count(@test_input), 4
    assert_equal get_shiny_cost(@test_input), 32
    puts 'tests pass'
  end
end

if __FILE__ == $PROGRAM_NAME
  aoc = Aoc07.new
  aoc.run
end
