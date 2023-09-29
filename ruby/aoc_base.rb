# frozen_string_literal: true

# Module for assertions methods
module Assertions
  AssertionError = Class.new(StandardError)

  def assert(condition, msg = nil)
    msg ||= 'Failed assertion.'
    raise AssertionError, msg unless condition

    true
  end

  def assert_equal(expected, actual, msg = nil)
    msg ||= "Expected #{expected.inspect} to equal #{actual.inspect}"
    assert(expected == actual, msg)
  end
end

# This is the Base Class for advent of code solutions.
# It reads the data from the files in they exists
class AocBase
  include Assertions
  INPUT_FILE = 'input.txt'
  TEST_FILE = 'test.txt'
  def initialize
    raise "Input File dosnt't exist" unless File.exist?(INPUT_FILE)

    File.open(INPUT_FILE) do |file|
      @input = file.read
    end
    return unless File.exist?(TEST_FILE)

    File.open(TEST_FILE) do |file|
      @test_input = file.read
    end
  end

  # Runs the tests and the solutions and print their results
  def run
    do_tests
    answer_one = part_one
    puts "Part 1 answer is #{answer_one}"
    answer_two = part_two
    puts "Part 2 answer is #{answer_two}"
  end

  protected

  # Solves part one and provide the answer
  def part_one
    raise NotImplementedError, "#{self.class} dons't implement part_1 method"
  end

  # Solves part two and provide the answer
  def part_two
    raise NotImplementedError, "#{self.class} dons't implement part_1 method"
  end

  # Tests can go here if they are needed
  def do_tests
    puts "#{self.class} doesn't have tests"
  end
end
